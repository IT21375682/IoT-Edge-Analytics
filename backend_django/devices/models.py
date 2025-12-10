from django.db import models

# Create your models here.
class Device(models.Model):
    name = models.CharField(max_length=255)
    device_type = models.CharField(max_length=50)
    status = models.CharField(max_length=20, default="active")
    last_seen = models.DateTimeField(auto_now=True)
    
    def __str__(self):
        return f"{self.name} ({self.device_type})"
    
class Telemetry(models.Model):
    device = models.ForeignKey(Device, on_delete=models.CASCADE)
    temperature = models.FloatField()
    humidity = models.FloatField()
    cpu_usage = models.FloatField()
    timestamp = models.DateTimeField(auto_now_add=True)

    def __str__(self):
        return f"Telemetry for {self.device.name} at {self.timestamp}"    