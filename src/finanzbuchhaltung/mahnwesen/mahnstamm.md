# Mahnstamm

<!-- source: https://amic.de/hilfe/mahnstamm.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Stammdaten > Mahnwesen einrichten > Funktion Mahnstamm **F6**

Direktsprung **[FIMSG]**.

Mahnstamm und Mahnsätze müssen immer gemeinsam eingerichtet werden, d.h. Wenn es zu einer Mahngruppe und Mahnstufe einen Datensatz im Mahnstamm existiert, muss für diese Kombination auch mindestens ein Eintrag in den Mahnsätzen existieren. Der Pfleger „[Mahnsätze einrichten](./mahnsaetze_einrichten.md)“ übernimmt dies automatisch und sollte diesem Pfleger vorgezogen werden.

| | Beschreibung |
| --- | --- |
| Mahngruppe | Angabe der Mahngruppe, für die die Bedingungen gelten  
 |
| Mahnstufe | Angabe der Mahnstufe, für die die Bedingungen gelten sollen, z.B. **"1"** für **"Mahnstufe 1"**  
 |
| Buchungstext | Ist hier ein Text eingegeben, so wird dieser bei der Übernahme der Mahngebühren in die Primanota verwendet, sonst der als [Einrichterparameter](./mahnungen_bearbeiten.md#MahnungenBuchen) hinterlegte Buchungstext „Text Hauptzeile bei Übernahme der Mahnungen in die Primanota“  
 |
| Formular-Id  
    
 | Nummer des Mahnformulars, das ausgedruckt werden soll. Es kann somit für jede Kombination aus Mahngruppe und Mahnstufe ein eigenes Formular mit unterschiedlichem Aufbau bzw. Text hinterlegt werden. Man kann aber auch für jede Stufe dasselbe Formular hinterlegen und die unterschiedlichen Mahnstufen durch den Mahntext kenntlich machen.  
 |
| Zinsgruppe | Falls Verzugszinsen berechnet werden sollen, wird hier die Zinsgruppe angegeben, deren Werte berücksichtigt werden sollen. Bei der Berechnung der Mahnzinsen wird nur der Soll-Zinssatz herangezogen.  
 |
| Mahnabstand | Der Mahnabstand zwischen zwei Mahnungen. Häufig wird von der Fälligkeit bis zur ersten Mahnung noch eine Schonfrist gewährt. In diesem Fall muss hier bei Mahnstufe 1 ein Zeitraum von z.B. 14 Tagen eingetragen werden, für Mahnstufe 2 und höher wird dann z.B. 10 Tage eingetragen. Somit sind auch unterschiedliche Intervalle je Stufe möglich.  
 |
| **Alle folgenden Felder erscheinen nur bei aktiver Belegversand-Lizenz** |
| Versandprofil | Hier steht das Versandprofil aus dem [Versandprofilstamm](../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/versandprofilstamm.md), das zur Versendung dieser Belege verwendet werden soll. Wird hier nichts angegeben, so wird für diese Mahngruppe keine Mahnung versendet.  
 |
| Formular bei Mail-Versand | Optional. Hier kann ein vom Druckformular abweichendes Formular eingerichtet werden. Ist kein Formular angegeben, wird das Mahnformular (s.o.) verwendet.  
 |
| Formular Mailbody | Mahnungen werden als Anhang versendet. Die eigentliche Mail kann hier als Formular definiert werden. Es stehen alle auch in der Mahnung vorhandenen Druckpositionen zu Verfügung. Die Betreffzeile der Mail kann in diesem Formular im Formularbereich „Mahnung Mail Betreffzeile“ definiert werden.  
 |
| Anstelle des Formulars für den Mailbody können auch Datenbankprozeduren verwendet werden. Ist in dem Feld „DB-Funktion Mailbody“ etwas eingetragen, dann werden die Prozeduren verwendet und das „Formular Mailbody“ wird ignoriert |
| FA-Eintrag Mailbody | FA-ID des Formulararchiv-Eintrags eines HTML-Body-Templates, das mit Hilfe der Body-Funktion zu einem HTML-Body verarbeitet werden kann.  
Der Eintrag hier ist optional, jedoch muss, wenn kein Template verwendet werden soll, die DB-Funktion das Dokument komplett aufbauen.  
 |
| DB-Funktion Mailbody | Wird hier eine Funktion hinterlegt, dann wird das „**Formular Mailbody**“ ignoriert und die Funktion liefert den Text der Mail. Sie erhält als Parameter die Mahnungid, die Adressid, und die auf der Druckmaske angegebenen Zahlungsdatum und Zahlungsfrist. Die Funktion muss einen Wert vom Typen long varchar zurückgeben.  
Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.  
   
   
**Wichtig:** Wenn die Funktion für den Mailbody einen Fehler liefert, wird kein Maildokument versendet. Bei Verwendung des Formulars für den Mailbody wird bei Fehlern ein Standard-Body „&lt;h1>Mahnung&lt;/h1>“ erzeugt.  
 |
| DB-Funktion Betreff | Diese Funktion wird nur ausgeführt, wenn auch die Funktion für den Mailbody angegeben wurde. Sie erhält als Parameter die Mahnungid, die Adressid, und die auf der Druckmaske angegebenen Zahlungsdatum und Zahlungsfrist. Die Funktion muss einen Wert vom Typen „long varchar“ zurückgeben. Auch wenn der Rückgabewert ein long Varchar ist, werden nur die ersten 255 Zeichen ausgewertet. In der F3-Auswahl werden nur Funktionen angeboten, die in der Datenbank angelegt sind und die entsprechenden Parameter haben.  
Gibt man den Namen einer nichtexistierenden Funktion an, um eine neue Funktion zu erstellen, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.  
Beispiel:  
 |
