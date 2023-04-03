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

# If modifying these scopes, delete the file token.json.
SCOPES = ['https://www.googleapis.com/auth/calendar.readonly']

# Month Constant 
MONTHS_LENGTH = [31,28,31,30,31,30,31,31,30,31,30,31]
MONTHS_LENGTH_LEAP = [31,29,31,30,31,30,31,31,30,31,30,31]

def main():

    """Shows basic usage of the Google Calendar API.
    Prints the start and name of the next 10 events on the user's calendar.
    """
    creds = None
    cont = True
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
        
    try:
        service = build('calendar', 'v3', credentials=creds)
        output_events(service)
        

    except HttpError as error:
        print('An error occurred: %s' % error)

# Asks user to input data for a specific date. Will try again if
# input is invalid due to use of other symbols besides numbers or
# date specified is not valid according to the Gregorian Calendar.
# Is used by other functions.
def inputDate():
    cont = True
    while(cont):
        try:
            year = input("Enter year: ")
            month = input("Enter month (in numbers): ")
            day = input("Enter day: ")
            fromDate = datetime.datetime(int(year),int(month),int(day),0,0,0).isoformat() + 'Z'
            toDate = datetime.datetime(int(year),int(month),int(day),23,59,59).isoformat() + 'Z'
            cont = False      
        except(ValueError):
            print('Error! Date is not valid.')
        except(TypeError):
            print('Error! Invalid input.')
            
    return fromDate,toDate


# Displays all events that happened on a day as specified by the user.
# Uses function inputDate to get date.
def output_events(service):

    file = open("calendar.txt", "w")

    fromDate,toDate = inputDate()
        
    events_result = service.events().list(calendarId='primary', timeMin=fromDate,timeMax=toDate,
                                            singleEvents=True,orderBy='startTime').execute()
    events = events_result.get('items', [])

    if not events:
        print('No upcoming events found.')
        return

    # Prints the start and name of the next 10 events
    for event in events:
        start = event['start'].get('dateTime', event['start'].get('date'))
        file.write(event['summary'] + "|" + start + "\n")
        
if __name__ == '__main__':
    main()