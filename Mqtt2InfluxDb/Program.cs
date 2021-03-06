
using InfluxDB.Client;
using InfluxDB.Client.Api.Domain;
using Microsoft.Extensions.Configuration;
using MQTTnet;
using MQTTnet.Client;
using MQTTnet.Extensions.ManagedClient;
using System.Text;

namespace Mqtt2InfluxDb
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Starting Mqtt2InfluxDb");
            
            IConfiguration config = new ConfigurationBuilder()
                .AddJsonFile("config/appsettings.json")
                .Build();

            var mqttFactory = new MqttFactory();
            
            
            Console.WriteLine("Connecting to Mqtt-Server " + config.GetSection("Mqtt:ServerUri").Value);

            using (var managedMqttClient = mqttFactory.CreateManagedMqttClient())
            {
                var mqttClientOptions = new MqttClientOptionsBuilder()
                    .WithTcpServer(config.GetSection("Mqtt:ServerUri").Value)
                    .Build();

                var managedMqttClientOptions = new ManagedMqttClientOptionsBuilder()
                    .WithClientOptions(mqttClientOptions)
                    .Build();
                
                
                Console.WriteLine("Connecting to InfluxDB-Server " + config.GetSection("InfluxDb:ServerUri").Value);

                var influxDbClient = InfluxDBClientFactory.Create(config.GetSection("InfluxDb:ServerUri").Value, config.GetSection("InfluxDb:Token").Value);
                var org = config.GetSection("InfluxDb:Org").Value;
                var bucket = config.GetSection("InfluxDb:Bucket").Value;

                var writeApiAsync = influxDbClient.GetWriteApiAsync();

                managedMqttClient.ApplicationMessageReceivedAsync += e =>
                {
                    var line = e.ApplicationMessage.Topic + " value=" + Encoding.UTF8.GetString(e.ApplicationMessage.Payload);
                    //Console.WriteLine(line);
                    writeApiAsync.WriteRecordAsync(line, WritePrecision.Ns, bucket, org);
                    return Task.CompletedTask;
                };

                managedMqttClient.StartAsync(managedMqttClientOptions);

                managedMqttClient.SubscribeAsync("#");

                while (true)
                {
                    Thread.Sleep(10000);
                }
            }
        }
    }
}
