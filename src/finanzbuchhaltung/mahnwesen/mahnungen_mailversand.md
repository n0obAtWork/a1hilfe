# Mahnungen Mailversand

<!-- source: https://amic.de/hilfe/mahnungenmailversand.htm -->

Mahnungen können so eingerichtet werden, dass zusätzlich zum Druck oder an Stelle des Drucks per Mail versendet werden können. Dazu müssen folgen Voraussetzungen gegeben sein:

1) Der Belegversand-Lizenz muss aktiv sein.

2) Ein [Versandprofil](../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/versandprofilstamm.md) muss eingerichtet sein.

3) In den Stammdaten des [Mahnstamm](./mahnstamm.md) müssen zusätzliche Felder gepflegt werden.

   - Ein abweichendes Formular für den Mailversand. Dieses Formular kann z.B. zusätzliche Grafiken enthalten, die bei der Druckversion nicht enthalten sind. In der F3-Auswahl werden nur Formulare angeboten, bei denen die Archivierung aktiviert ist. Wird hier kein Formular hinterlegt, dann wird das beim Druck angegebene verwendet.
   - Zur Steuerung des Mailbodys für die eigentliche Mail kann entweder ein HTML-Formular oder eine [Datenbankfunktion](./mahnstamm.md#MahnStammMailVersand), die den HTML-Aufbau übernimmt, verwendet werden. In dem Formular müssen HTML-Tags für die Formatierung verwendet werden. Hier existiert ein Formular mit der Nummer -1120, das so wie es ist verwendet werden kann oder als Vorlage benutzt werden kann. In diesem Formular stehen alle Felder und Bereiche der Standard Mahnung zur Verfügung. Zusätzlich existiert auch ein Bereich „Mahnung Betreffzeile“, in dem man die Betreff-Zeile der Mail einrichten kann. Ist kein Formular eingerichtet, erscheint als Betreff und als Mailinhalt lediglich der Text „Mahnung“.  
    
**HINWEIS:** *Um Grafiken in das Formular mit einzubinden, kann man den bekannten HTML-Syntax &lt;img src="cid:XXXXXX" alt="mein bild" /> verwenden. Für XXXXXX muss die GUID aus dem Formulararchiv, in dem die Grafik hinterlegt sein muss, angegeben werden.  
*  

   - Ist das Versandprofil nicht eingerichtet, wird für alle Personenkonten mit diesem Mahnsatz kein Mailversand durchgeführt.  
    

4) In den Hauptanschriften oder den Ansprechpartnern muss eine Mailadresse für Mahnungen eingerichtet sein. Dazu wählt man in der Auswahlliste „Anschriften“ **[ANSCH]** in der Variante „Ansprechpartner“ die Anschrift des Kunden, der man die Mail zuordnen will, mit **F5** zum Bearbeiten aus. Dies kann die Hauptadresse sein oder die eines Ansprechpartners. Die Adressdaten, die im HTML-Formular verwendet werden, beziehen sich immer auf die Adresse, an die auch die Mail geht, während in der Mahnung selber nach wie vor die Hauptadresse verwendet wird.

   Zu diesen Mailadressen muss man dann noch angeben werden, ob sie „mit Belegdruck“ oder „statt Belegdruck“ versendet werden sollen. Gibt man „nein“ an, wird die eingetragene Mail nicht verwendet.

   ![](../../ImagesExt/image8_640.png)  
    
An wen die Mails für einen Kunden gehen, kann in den Fibu-Merkmalen eingesehen werden.  
    

5) Es existiert ein Steuerparameter „Mailversand bestätigen“ in der Gruppe „Optionen Finanzwesen“, der steuert, ob erst eine Liste mit den zu versendenden Mahnungen geöffnet wird um dort noch einmal zu kontrollieren oder ggf. eine Kurzliste zu drucken. Dieser steht standartmäßig auf **Nein**. Hat man diesen Steuerparameter auf **Ja** gestellt, so kann man mit der Funktion „***Daten übernehmen***“ den Mailversand für die markierten Nachrichten sofort starten. Nachrichten, die nicht markiert wurden, werden nicht versendet, bleiben aber weiterhin erhalten. In der Anwendung „Mailversand Finanzbuchhaltung“ (Direktsprung **[FMV]**) sind diese als „offene Versandaufträge“ zu finden. Sie können hier angesehen oder nachträglich versendet werden.

Ist die Belegversandlizenz aktiv und ist eine Mailadresse für Mahnungen eingerichtet worden, so erscheint bei der Druckerabfrage neben dem „Druck“-Button ein weiterer Button mit der Aufschrift „Versand/Druck“. Beim Betätigen des „Druck“-Buttons wird lediglich die Mahnung gedruckt. Beim „Versand/Druck“-Button wird die Mahnung je nach Einstellung per Mail versendet. Am Ende erscheint ein Bildschirm, in dem alle Kunden aufgelistet werden, die eine Mail erhalten haben. Konnte keine Mail versendet werden, so erscheint die Meldung „Keine Mail versendet.“
