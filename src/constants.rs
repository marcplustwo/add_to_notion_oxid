pub const INSTRUCTIONS_MSG: &str = "
*AddToNotionBot*
I am your addToNotionBot I can add resources directly into your Notion page
To know which Notion account and page to add to, I need some information from you

__Preparation__
Step 1:
I need an \"Internal Integration Token\"
Go to [My Integrations](https://www\\.notion\\.so/my\\-integrations) and create an Integration
I will ask for the \"Internal Integration Token\" shortly

Step 2:
This Bot creates pages in a Database, you need to Create a *\\(full\\-page\\) database with the properties: _Name, URL, Tags, Image_*

Step 3:
Go to the page you want to use as a WebDump, go to settings 
Then you need to add the connection to the connection you created in Step 1
Next, you have to send me the page link so I know where to add your content

These steps are explained again here:
[Create a Notion Integration](https://developers\\.notion\\.com/docs/create\\-a\\-notion\\-integration)

// __Help__
// Use the command `/help` to show this message
";
