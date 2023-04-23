
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
import sys

# If modifying these scopes, delete the file token.json.
SCOPES = ['https://www.googleapis.com/auth/gmail.modify','https://www.googleapis.com/auth/calendar']
# Month Constant 
MONTHS_LENGTH = [31,28,31,30,31,30,31,31,30,31,30,31]
MONTHS_LENGTH_LEAP = [31,29,31,30,31,30,31,31,30,31,30,31]

def credentials():
    creds = None

    # The file token.json stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists('../token.json'):
        creds = Credentials.from_authorized_user_file('../token.json', SCOPES)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(
                '../client.json', SCOPES)
            creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        with open('../token.json', 'w') as token:
            token.write(creds.to_json())
    return (creds)


# some accessor functions
# returns profile
def getProfile(creds):
    try:
        service = build('calendar', 'v3', credentials=creds)
        return service
    except Exception as error:
        print('An error occurred: %s' % error)
        
# Asks user to input data for a specific date. Will try again if
# input is invalid due to use of other symbols besides numbers or
# date specified is not valid according to the Gregorian Calendar.
# Is used by other functions.
def inputDate(year, month, day):
    try:
        fromDate = datetime.datetime(int(year),int(month),int(day),0,0,0).isoformat() + 'Z'
        toDate = datetime.datetime(int(year),int(month),int(day),23,59,59).isoformat() + 'Z'     
    except(ValueError):
        print('Error! Date is not valid.')
    except(TypeError):
        print('Error! Invalid input.')
            
    return fromDate,toDate


# Displays all events that happened on a day as specified by the user.
# Uses function inputDate to get date.
def displayEvents(service, fromDate, toDate):
        
    file = open("calendar.txt", "w")
    events_result = service.events().list(calendarId='primary', timeMin=fromDate,timeMax=toDate,
                                            singleEvents=True,orderBy='startTime').execute()
    events = events_result.get('items', [])

    if not events:
        file.write('Today|No upcoming events found.|\n')
        return

    # Prints the start and name of the next 10 events
    for event in events:
        start = event['start'].get('dateTime', event['start'].get('date'))
        file.write(event['summary'] + "|" + start + "|\n")

year = sys.argv[1]
month = sys.argv[2]
day = sys.argv[3]
fromDate, toDate = inputDate(year, month, day)      
creds = credentials()
service = getProfile(creds)
displayEvents(service,fromDate,toDate)