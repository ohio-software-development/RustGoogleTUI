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
        
    while(cont):
        try:
            print('Find All Events in a Date\n')
            service = build('calendar', 'v3', credentials=creds)
            displayEvents(service)
            

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
def displayEvents(service):
    fromDate,toDate = inputDate()
        
        # Call the Calendar API
    now = datetime.datetime.utcnow().isoformat() + 'Z'  # 'Z' indicates UTC time
            
    print('Getting the upcoming events')
    events_result = service.events().list(calendarId='primary', timeMin=fromDate,timeMax=toDate,
                                            singleEvents=True,orderBy='startTime').execute()
    events = events_result.get('items', [])

    if not events:
        print('No upcoming events found.')
        return

    # Prints the start and name of the next 10 events
    for event in events:
        start = event['start'].get('dateTime', event['start'].get('date'))
        print(start, event['summary'])
        pptest = event['start'].get('dateTime')
        print(pptest)
        
# is_leap
# Checks if the year input is a leap year
# returns true if it is and false otherwise 
def is_leap(year):
    if (year % 100 == 0):
        if (year % 400):
            return True
    if (year % 4 == 0):
        return True 
    return False 

# getOneWeeksDate
# Get a weeks worth of dates
# returns the dates
def getOneWeeksDate():
    now = time.localtime()
    now_string = time.strftime("%D", now)
    minute_string = time.strftime("%H:%M:%S", now)
    
    start_day = now.tm_mday
    start_month = now.tm_mon
    start_year = now.tm_year

    months = MONTHS_LENGTH
    if (is_leap(start_year)):
        months = MONTHS_LENGTH_LEAP

    end_day = start_day + 7
    end_month = start_month
    end_year = start_year
    if (end_day > months[start_month-1]):
        if (start_month < 12):
            difference = end_day - months[start_month-1] 
            end_month = start_month + 1
            end_day = difference
        else:
            difference = end_day - months[0] 
            end_month = 1 
            end_day = difference

    start = datetime.datetime(start_year, start_month, start_day, 0,0,0).isoformat()
    end = datetime.datetime(end_year, end_month, end_day, 0,0,0).isoformat()
    return start, end

if __name__ == '__main__':
    main()    
