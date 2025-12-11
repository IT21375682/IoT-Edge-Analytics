# worker/worker.py

import sys
import os
from celery import Celery

# Add the backend_django directory to the sys.path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'backend_django'))

# Ensure that the 'DJANGO_SETTINGS_MODULE' is set correctly
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'visiontrack.settings')

app = Celery('visiontrack')

# Load settings from Django settings.py
app.config_from_object('django.conf:settings', namespace='CELERY')

# Auto-discover tasks in Django apps
app.autodiscover_tasks()
