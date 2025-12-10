from rest_framework import serializers
from .models import Device, Telemetry

class DeviceSerializer(serializers.ModelSerializer):
    class Meta:
        model = Device
        fields = ['id', 'name', 'device_type', 'status', 'last_seen']
        
class TelemetrySerializer(serializers.ModelSerializer):
    class Meta:
        model = Telemetry
        fields = ['id', 'temperature', 'humidity', 'cpu_usage', 'timestamp', 'device', 'processed']    