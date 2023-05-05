# ------------------------------------------------------------
# File to save events during week into calendar.txt
# ------------------------------------------------------------

from __future__ import print_function

import datetime
import os.path
import numbers

import time 

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError
from pathlib import Path

# If modifying these scopes, delete the file token.json.

SCOPES = ['https://www.googleapis.com/auth/gmail.modify','https://www.googleapis.com/auth/calendar']
    
def credentials():
    creds = None

    # The file token.json stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists('token.json'):
        creds = Credentials.from_authorized_user_file('token.json', SCOPES)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(

                'client.json', SCOPES)
            creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        with open('token.json', 'w') as token:
            token.write(creds.to_json())
    return (creds)


# some accessor functions
# returns profile
def getProfile(creds):
    # Connect to the Gmail API
    service = build('calendar', 'v3', credentials=creds)
    try:
        return service
    except Exception as error:
        print('An error occurred: %s' % error)

# Displays all events that happened on a day as specified by the user.
# Uses function inputDate to get date.

def output_days_events(service):

    file = open("calendar.txt", "w")
    
    # now = datetime.datetime.utcnow().isoformat() + 'Z'  # 'Z' indicates UTC time
    # print('Getting the upcoming 10 events')
    # events_result = service.events().list(calendarId='primary', timeMin=now,
    #                                         maxResults=10, singleEvents=True,
    #                                         orderBy='startTime').execute()

    
    
    fromDate = datetime.datetime.now().astimezone().isoformat()
    
    toDate = (datetime.datetime.now().astimezone() + datetime.timedelta(days=7)).isoformat()
    
    events_result = service.events().list(calendarId='primary', timeMin=fromDate,timeMax=toDate,
                                            singleEvents=True,orderBy='startTime').execute()
    events = events_result.get('items', [])
    
    if not events:
        file.write('|No upcoming events found.|\n')
        return

    for event in events:
        start = event['start'].get('dateTime', event['start'].get('date'))
        file.write(event['summary'] + "|" + start + "|\n")
        
os.chdir(str(Path.home()) + "/RustGoogleTUI/RustTUIOUSDC/src/")
creds = credentials()
service = getProfile(creds)
try:
    output_days_events(service)

except HttpError as error:
    print('An error occurred: %s' % error)
