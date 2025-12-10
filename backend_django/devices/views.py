from django.shortcuts import render
from rest_framework import viewsets
from .models import Device, Telemetry
from .serializers import DeviceSerializer, TelemetrySerializer


# Create your views here.
class DeviceViewSet(viewsets.ModelViewSet):
    queryset = Device.objects.all()
    serializer_class = DeviceSerializer
    
class TelemetryViewSet(viewsets.ModelViewSet):
    queryset = Telemetry.objects.all()
    serializer_class = TelemetrySerializer    
