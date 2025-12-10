from django.shortcuts import render
from rest_framework import viewsets
from .models import Device, Telemetry
from .serializers import DeviceSerializer, TelemetrySerializer
from .tasks import process_telemetry_data


# Create your views here.
class DeviceViewSet(viewsets.ModelViewSet):
    queryset = Device.objects.all()
    serializer_class = DeviceSerializer
    
class TelemetryViewSet(viewsets.ModelViewSet):
    queryset = Telemetry.objects.all()
    serializer_class = TelemetrySerializer 
    
   
    def perform_create(self, serializer):
        # Save telemetry data
        telemetry = serializer.save()

        # Trigger the background task to process the telemetry data
        process_telemetry_data.delay(telemetry.id)  
