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
    # Connect to the Gmail API
    service = build('calendar', 'v3', credentials=creds)
    try:
        return service
    except Exception as error:
        print('An error occurred: %s' % error)


def addEvents(service, fromDate, toDate, summary, description, atList):
    
    event = {
    'summary': summary,
    'description': description,
    'start': {
        'dateTime': fromDate,
        'timeZone': 'America/New_York',
    },
    'end': {
        'dateTime': toDate,
        'timeZone': 'America/New_York',
    },
    'recurrence': [
        'RRULE:FREQ=DAILY;COUNT=2'
    ],
    'attendees': atList,
    'reminders': {
        'useDefault': False,
        'overrides': [
        {'method': 'email', 'minutes': 24 * 60},
        {'method': 'popup', 'minutes': 10},
        ],
    },
    }
    
    
    event = service.events().insert(calendarId='primary', body=event).execute()
    return

def inputDate(year, month, day, fromHour, fromMinute, endYear, endMonth, endDay, toHour, toMinute):

    try:     
        fromDate = datetime.datetime(int(year),int(month),int(day),int(fromHour),int(fromMinute),00).isoformat()
        toDate = datetime.datetime(int(endYear),int(endMonth),int(endDay),int(toHour),int(toMinute),00).isoformat()
        if(fromDate > toDate):
            raise Exception('Error! Dates are not in bound.')     
    except(ValueError):
        print('Error! Date is not valid.')
    except(TypeError):
        print('Error! Invalid input.')
            
    return fromDate,toDate


year = sys.argv[1]
month = sys.argv[2]
day = sys.argv[3]
fromHour = sys.argv[4]
fromMinute = sys.argv[5]

endYear = sys.argv[6]
endMonth = sys.argv[7]
endDay = sys.argv[8]
toHour = sys.argv[9]
toMinute = sys.argv[10]

summary = sys.argv[11]
description = sys.argv[12]
atList = []
i = 13
#while(sys.argv[i] != 'Q'):
#    atList.append[sys.argv[i]]
#    i += 1

fromDate, toDate = inputDate(year, month, day, fromHour, fromMinute, endYear, endMonth, endDay, toHour, toMinute)      
creds = credentials()
service = getProfile(creds)
addEvents(service, fromDate, toDate, summary, description, atList)