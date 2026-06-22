# Email senden

<!-- source: https://amic.de/hilfe/_emailsenden.htm -->

Aus den Auswahllisten Kunden, Anschriften, Lieferanten und Kontokorrentkunden ist die Funktion Email senden verfügbar. Mit Hilfe dieser Funktion kann eine E-Mail an die markierten Kunden (Mailadresse aus der Kundenhauptanschrift) oder eine Mailadresse aus der Anschrift versendet werden.

Zu diesem Zweck wird über ein VBA-Skript der Mailclient Outlook geöffnet und die Mailadressen werden als Liste in den TO bzw. BCC.Bereich eingefügt. Das Ziel ist in einer kurzen Abfrage zu definieren.

Steht Outlook als Mailclient nicht zur Verfügung, so kann ein anderes VBA-Skript definiert werden, das den Versand übernimmt. Zu diesem Zweck ist der Einrichterparameter in [Archiv Mail Versand](../dokumentenverwaltung/archiv_manager/archiv_mail_versand/index.md) anzupassen.
