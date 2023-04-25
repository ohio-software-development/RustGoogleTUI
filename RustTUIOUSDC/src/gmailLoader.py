from __future__ import print_function

import os.path
from bs4 import BeautifulSoup
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError
import base64
from email.message import EmailMessage
import email
import sys

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
    service = build('gmail', 'v1', credentials=creds)
    try:
        return service.users().getProfile(userId='me').execute()
    except Exception as error:
        print('An error occurred: %s' % error)

# returns messages (broken)
def getMessages(creds):
    service = build('gmail', 'v1', credentials=creds)
    try:
        return service.users().messages().list(userId='me')
    except Exception as error:
        print('An error occurred: %s' % error)

#this creates a mime_message that we parse to get the valid email content
def get_mime_message(service, msg_id):
  try:
    message = service.users().messages().get(userId='me', id=msg_id,
                                             format='raw').execute()
    #print('Message snippet: %s' % message['snippet'])
    #print('Message snippet: %s' % message['threadId'])

    msg_str = base64.urlsafe_b64decode(message['raw'].encode("utf-8")).decode("utf-8")
    mime_msg = email.message_from_string(msg_str)
    
    print("To: " + mime_msg['to'])
    print("From: " + mime_msg['from'])
    print("Subject: " + mime_msg['Subject'])
    print("Body: ")
    for x in mime_msg.walk():
        if x.get_content_maintype() == 'text':
            print(x)
            break
    
    return mime_msg
  except Exception as error:
    print('An error occurred: %s' % error)


def getSnippet(service, msg_id):
    try:
        message = service.users().messages().get(userId='me', id=msg_id, format='raw').execute()
        return message['snippet']
    except Exception as error:
        print('An error occurred: %s' % error)


#credentials for OAUTH
creds = credentials()

lst = getMessages(creds).execute()
x = 0

numMail = 10

# option to load more mail
if len(sys.argv) > 1:
    numMail = sys.argv[1]

f = open("./description.txt", 'w')

# output numMail mails
while x < int(numMail):
    key = "APIMAIL#" + str(x) + "\n"
    #to_insert = get_mime_message(build('gmail','v1', credentials=creds), lst['messages'][x]['id'])
    to_insert = getSnippet(build('gmail','v1', credentials=creds), lst['messages'][x]['id'])
    f.write(key)
    f.write(str(to_insert))
    x+=1

# Python3 gmailCleanAPI.py