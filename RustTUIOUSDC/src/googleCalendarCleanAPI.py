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
SCOPES = ['https://www.googleapis.com/auth/gmail.modify','https://www.googleapis.com/auth/calendar']

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
            service = build('calendar', 'v3', credentials=creds)
            displayEvents(service)
            addEvents(service)
            cont = False

        except HttpError as error:
            print('An error occurred: %s' % error)

# Asks user to input data for a specific date. Will try again if
# input is invalid due to use of other symbols besides numbers or
# date specified is not valid according to the Gregorian Calendar.
# Is used by other functions.
def inputDate(i = 0):
    cont = True
    while(cont):
        try:
            year = input("Enter year: ")
            month = input("Enter month (in numbers): ")
            day = input("Enter day: ")
            if(i == 0):
                fromDate = datetime.datetime(int(year),int(month),int(day),0,0,0).isoformat() + 'Z'
                toDate = datetime.datetime(int(year),int(month),int(day),23,59,59).isoformat() + 'Z'
            else:
                endYear = year
                endMonth = month
                endDay = day
                
                fromHour = input("Enter starting hour: ")
                fromMinute = input("Enter starting minute: ")
                
                ask = input("Does event have different starting date? (Y/N): ")
                while(ask.upper() != "Y" and ask.upper() != "N"):
                    ask = input("Error! Invalid Input. Does event have different starting date? (Y/N): ")
                if(ask.upper() == "Y"):
                    endYear = input("Enter end year: ")
                    endMonth = input("Enter end month: ")
                    endDay = input("Enter end day: ")
                
                
                toHour = input("Enter ending hour: ")
                toMinute = input("Enter ending minute: ")
                
                fromDate = datetime.datetime(int(year),int(month),int(day),int(fromHour),int(fromMinute),00).isoformat()
                toDate = datetime.datetime(int(endYear),int(endMonth),int(endDay),int(toHour),int(toMinute),00).isoformat()
                if(fromDate > toDate):
                    raise Exception('Error! Dates are not in bound.')
                
            cont = False      
        except(ValueError):
            print('Error! Date is not valid.')
        except(TypeError):
            print('Error! Invalid input.')
            
    return fromDate,toDate


# Displays all events that happened on a day as specified by the user.
# Uses function inputDate to get date.
def displayEvents(service):
    print('Find All Events in a Date\n')
    
    fromDate,toDate = inputDate()
        
    #now = datetime.datetime.utcnow().isoformat() + 'Z'  # 'Z' indicates UTC time
    
    print('Getting the upcoming events')
    events_result = service.events().list(calendarId='primary', timeMin=fromDate,timeMax=toDate,
                                            singleEvents=True,orderBy='startTime').execute()
    events = events_result.get('items', [])

    if not events:
        print('No upcoming events found.')
        return

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


def addEvents(service):
    print('Add an Event in a Date\n')
    summary = input("Enter Title of event: ")
    description = input("Enter Description: ")
    atList = []
    
    attendees = input("Enter email of an attendee (Enter q to skip): ")
    while(attendees.upper() != 'Q'):
        atList.append({'email': attendees})
        attendees = input("Enter email of an attendee (Enter q to skip): ")
        
    fromDate, toDate = inputDate(1)
    addInfo = input("Add Additional Information (Y/N): ")
    
    event = {
    'summary': summary,
    'description': description,
    'start': {
        'dateTime': fromDate,
        'timeZone': 'UTC',
    },
    'end': {
        'dateTime': toDate,
        'timeZone': 'UTC',
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
    print ('Event created: %s' % (event.get('htmlLink')))
    return

if __name__ == '__main__':
    main()    
