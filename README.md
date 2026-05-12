1. How much data your publisher program will send to the message broker in one run?

    The amount of data the publisher program will send to the message broker in one run depends on the implementation of the publisher. It could be a single message or multiple messages, depending on how the publisher is designed to operate. In main.rs, the publisher is sending five messages to the message broker, each containing a `UserCreatedEventMessage` with a user ID and user name. The total amount of data sent will depend on the size of each message, which can be calculated based on the structure of the `UserCreatedEventMessage` and the content of the user ID and user name fields.

2. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?

    The URL "amqp://guest:guest@localhost:5672" is a connection string that specifies how to connect to the message broker (RabbitMQ in this case). It includes the protocol (amqp), the username (guest), the password (guest), the host (localhost), and the port (5672). This means that both the publisher and subscriber programs are connecting to the same RabbitMQ server using the same credentials and connection details.

Screenshots:
![Screenshot of running RabbitMQ](assets/running_RabbitMQ_as_message_broker.png)

![Sending and processing event](assets/sending_and_processing_event.png)
Here is what happens when I ran the publisher program: It sends five messages to the RabbitMQ server, each containing a `UserCreatedEventMessage` with a user ID and user name. The output shows that each message has been published successfully.

![Monitoring chart based on publisher](assets/monitoring_chart_based_on_publisher.png)
The spikes on the chart indicate the times when the publisher program sent messages to the RabbitMQ servera nd each spike corresponds to a batch of messages being published. The chart helps visualize the activity of the publisher and how it interacts with the message broker over time.