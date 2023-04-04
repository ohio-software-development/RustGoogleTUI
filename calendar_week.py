
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
        
    open("calendar.txt", "w")
    try:
        service = build('calendar', 'v3', credentials=creds)
        days = get_one_weeks_date()
        for i in range(len(days)):
            output_days_events(service, days[i])

    except HttpError as error:
        print('An error occurred: %s' % error)

# Displays all events that happened on a day as specified by the user.
# Uses function inputDate to get date.
def output_days_events(service, day):

    file = open("calendar.txt", "a")

    fromDate = day.isoformat() + 'Z'
    toDate = day.replace(hour = 23, minute = 59, second = 59).isoformat() + 'Z'
        
    events_result = service.events().list(calendarId='primary', timeMin=fromDate,timeMax=toDate,
                                            singleEvents=True,orderBy='startTime').execute()
    events = events_result.get('items', [])

    if not events:
        file.write(day.isoformat() + "|" + 'No upcoming events found.\n')
        return

    # Prints the start and name of the next 10 events
    for event in events:
        start = event['start'].get('dateTime', event['start'].get('date'))
        file.write(day.isoformat() + "|" + event['summary'] + "|" + start + "\n")
        
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
def get_one_weeks_date():
    now = time.localtime()

    start_year = now.tm_year
    months = MONTHS_LENGTH
    if (is_leap(start_year)):
        months = MONTHS_LENGTH_LEAP

    last = datetime.datetime.today()
    last.replace(minute = 0, second = 0, hour = 0)
    days = [last]
    for i in range(1, 7):
        new_day = last.day + 1
        new_month = last.month 
        new_year = last.year 
        if (new_day > months[last.month-1]):
            if (last.month < 12):
                new_day = 1
                new_month = last.month + 1
            else:
                new_day = 1
                new_month = 1
                new_year = last.year + 1
        last = last.replace(day = new_day, month = new_month, year = new_year)
        days.append(last)
        
    return days

if __name__ == '__main__':
    main()
