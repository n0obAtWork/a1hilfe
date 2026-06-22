# Funktionen

<!-- source: https://amic.de/hilfe/_mailversandfunktioneninmail.htm -->

| **Funktionen** |
| --- |
| Freigeben/Versenden | Gibt Einträge frei zur Versendung. Wenn der Versand [Synchron](../../mailversand_allgemein/einrichtung_mailversand/synchron_oder_asynchron/index.md) erfolgen soll, so werden die E-Mails auch sofort versendet. Anderenfalls werden sie zyklisch durch den Dienst versendet.  
Bei E-Mails mit Mailversandquelle = Ware-Beleg oder Mailversandquelle = Eohware-Sammeldruck wird bei erfolgreichem Versand das Kennzeichen V_StatusBelegVersand im Vorgangstamm beziehungsweise V_RohwareStatusMail in der Relation V_Rohware auf den Wert ‚Versendet‘ gesetzt. |
| Zurückstellen | Stellt den Eintrag zurück. Dies kann eine bewusste Rückstellung zur späteren Klärung sein.  
Bereits versendete Einträge lassen sich nicht zurückstellen. |
| Löschen | Löscht den Eintrag  
Bereits versendete Einträge lassen sich nicht löschen  
Bei E-Mails mit Mailversandquelle = Ware-Beleg oder Mailversandquelle = Eohware-Sammeldruck wird bei erfolgreichem Löschen das Kennzeichen V_StatusBelegVersand im Vorgangstamm beziehungsweise V_RohwareStatusMail in der Relation V_Rohware auf den Wert ‚Zurück genommen‘ gesetzt. |
| Email ändern | Öffnet einen Pfleger zum nachträglichen Bearbeiten der Mailadressen. Wenn die Mail bereits versendet wurde, wird ein neuer Eintrag erzeugt. Der Inhalt bleibt der Originalinhalt. Der Verweis zeigt auf die ursprüngliche Mail und nicht auf die ursprüngliche Quelle.  
Wenn eine Mail mit bereits gelöschter Verpostung erneut versendet werden soll, muss beim Ändern eine neue Verpostung ausgewählt werden. |
| Bereich | Filter der Auswahlliste |
