# devices/tasks.py

from celery import shared_task
from .models import Telemetry

@shared_task
def process_telemetry_data(telemetry_id):
     # This task will run in the background when called
    telemetry = Telemetry.objects.get(id=telemetry_id)
    # Process the telemetry data here
    
    if telemetry.temperature >50:
        print(f"Alert! High temperature recorded: {telemetry.temperature}")
     
    # Mark telemetry as processed
    telemetry.processed = True
    telemetry.save()    
    
    return f"Processed telemetry data with ID {telemetry_id}"