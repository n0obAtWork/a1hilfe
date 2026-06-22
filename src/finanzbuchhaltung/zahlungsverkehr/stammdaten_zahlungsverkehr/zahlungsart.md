# Zahlungsart

<!-- source: https://amic.de/hilfe/zahlungsart.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Stammdaten > Zahlungsarten

Direktsprung **[FIZAH]**.

Die Zahlungsart ist ein im Kunden- und Lieferantenstamm eingetragenes Kennzeichen, über das gesteuert wird, wie die Zahlung im automatischen Zahlungsverkehr bei Ein- und Ausgang erfolgen soll.

![Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. KI-generierte Inhalte können fehlerhaft sein.](../../../ImagesExt/image8_642.png "Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. KI-generierte Inhalte können fehlerhaft sein.")

| | Beschreibung |
| --- | --- |
| Nummer | Eindeutige Nummer der Zahlungsart, wie sie später im Kunden/Lieferantenstamm hinterlegt wird. Die nächste freie Nummer wird vorgeschlagen.  
 |
| Formularklasse | Wahlweise „Zahlungseingang“ oder „Zahlungsausgang“. Eine Auswahl mit **F3** ist möglich.  
Beim Wechseln der Formularklasse wird im Feld eRechnung Zahlungsweg der sinnvollste eRechnungs-Zahlungsweg neu vorbelegt.  
 |
| Bezeichnung | Bezeichnung der Zahlungsart zur einfacheren Identifikation in Auswahllisten oder F3-Auswahlen.  
Ist der Steuerungsparameter 34 "Mehrsprachigkeit aktiv“ in A.eins gesetzt, so hat man auf diesem Feld die Möglichkeit mit F3 [sprachabhängige Bezeichnungen](../../../firmenstamm/a_eins_sprache/sprachabhaengige_bezeichnung_in_den_stammdaten.md) zu pflegen.  
 |
| Skontierbar | Hier kann der Skontotyp eingetragen werden. Eine Auswahl mit **F3** ist möglich  
   
• **immer Skonto**: Skonto wir unabhängig vom Skontodatum immer gewährt/gezogen  
• **nie Skonto:** Selbst, wenn im Beleg Skonto vorgesehen ist und die Skontofrist noch nicht abgelaufen ist, wird kein Skonto gewährt.  
• **Abzug gem. Datum**: Dies ist die Vorbelegung. Skonto wird dann gewährt, wenn die Frist noch nicht abgelaufen ist.  
 |
| Skontierbar bei Verrechnung | Werden Rechnungen mit Gutschriften verrechnet, so kann es wünschenswert sein, bei den Gutschriften Skonto anders zu behandeln. Ist hier kein Wert eingetragen, so wird bei Gutschriften der Wert, der bei „Skontierbar“ eingetragen ist, verwendet.  
 |
| DTA-Typ | Zahlungsart bei Zahlung per Datenträgeraustausch. Der DTA-Typ wird nur bei der Formularklasse „Zahlungseingang“ abgefragt bzw. im Datenträgeraustausch verwendet. Bei Zahlungsausgang wird dieses Feld ausgeblendet. Der Hausbank muss beim DTA mitgeteilt werden, ob es sich bei den Lastschriften um eine **Einzugsermächtigung** oder um eine **Abbuchung** handelt.  
   
Hinweis: *Für das SEPA-Verfahren wird der Typ beim Mandat hinterlegt.*  
 |
| Echtzeitüberweisung | Dieses Feld wird nur für Formularklasse „Zahlungsausgang“ abgefragt. Wir hier **Ja** eingetragen, so entfällt beim Erstellen der Zahlungsvorschläge die Vorlauf Frist von einem Tag und als Ausführungsdatum wird der verwendete Stichtag verwendet. Es werden bei der Berechnung keine Bankarbeitstage mehr berücksichtigt, da Echtzeitüberweisungen auch am Wochenende und an Feiertagen ausgeführt werden.  
 |
| eRechnung Zahlungsweg  
    
 | Dieses Feld ist nur sichtbar, wenn die eRechnungs-Lizenz erworben wurde.  
    
In diesem Feld steht der Zahlungsweg, der beim eRechnungs-Export für diese Zahlungsart gewählt werden soll.  
Die Vorbelegung erfolgt anhand der Formularklasse. Bei Zahlungseingang wird der eRechnungs-Zahlungsweg mit Bankeinzug vorbelegt, bei Zahlungsausgang, wird er mit Überweisung vorbelegt.  
 |
| OP-Raffung Zahlungsverkehr | • **Standard Raffung**. Alle OPs werden wie bisher in einem Zahlungsbeleg zusammengefasst.  
• **Einzel – OP.** Es wird pro OP ein Zahlungsbeleg erstellt.  
 |
| DB-Prozedur VWZ  
 | Hier kann man eine private Datenbankprozedur hinterlegen, die den Verwendungszwecktext individuell zusammengestellt. Sie hat drei Parameter:  
   
1. die csatz_id aus der Tabelle AMIC_DTAUS_CSATZ  
    
    
2. die Arte des DTA-Verfahrens. Mögliche Werte sind:  
#define DTAVERFAHREN_UNDEF -1#define DTAVERFAHREN_NODTA 0#define DTAVERFAHREN_DTA 1#define DTAVERFAHREN_DTINT 2#define DTAVERFAHREN_DTAKASSE 8#define DTAVERFAHREN_SEPA 16  
3. die Anzahl der erlaubten Zeilen. Im DTA sind maximal 13 Zeilen a 27 Zeichen zulässig. Werden diese Grenzen überschritten, so wird der Rest ignoriert.  
   
Die Prozedur muss ein Resultset vom Typen Character der Länge 27 zurückgeben. Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.  
Eine Datenbankprozedur zur Erstellung des Verwendungszwecks könnte so aussehen:  
   
**Wichtig:** Das Feld Zahlungsavise in der Tabelle Zahlungsbeleg muss auf 1 gesetzt werden, um dem Programm mitzuteilen, dass eine Avise gedruckt werden muss. Ansonsten muss das Feld auf 0 gesetzt werden.  
 |
| DB-Funktion VWZ SEPA  
 | Dieses Feld erscheint nur, wenn der Steuerungsparameter „DTA-Ausgabeformat“ auf „SEPA“ steht. Hier kann man eine private Datenbankfunktion hinterlegen, die den Verwendungszwecktext individuell zusammengestellt. Sie erhält als Parameter die csatz_id aus der Tabelle AMIC_DTAUS_CSATZ. Die Funktion muss einen Wert vom Typen Character zurückgeben. SEPA unterstützt nur Verwendungszwecktexte bis zu einer Länge von 140 Zeichen. Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.  
Eine Datenbankfunktion zur Erstellung des Verwendungszwecks könnte so aussehen:  
   
**Wichtig:** Das Feld Zahlungsavise in der Tabelle Zahlungsbeleg muss auf 1 gesetzt werden, um dem Programm mitzuteilen, dass eine Avise gedruckt werden muss. Ansonsten muss das Feld auf 0 gesetzt werden.  
 |
| SEPA-Purpose-Code(TextSchlüssel) | Hier können nur Daten eingegeben werden, die in der Tabelle [SEPAPurposeCode](./sepa_purpose_code.md) hinterlegt wurden. Eine Auswahl mit **F3** ist möglich  
**ACHTUNG:** Ist in diesem Feld ein Wert eingetragen, so wird beim SEPA-Verfahren dieser Wert mit übertragen.  
 |
| DTINT-Verfahren trotz SEPA | Dieses Feld erscheint nur, wenn der Steuerungsparameter „DTINT-Verfahren aktiv“ auf **Ja** steht. Ob Belege im [SEPA-Verfahren](../sepa/index.md) abgewickelt werden wird Anhand von diversen Einstellungen entschieden. Setzt man dieses Feld jedoch auf **Ja**, werden alle diese Einstellungen ignoriert und die Zahlungsbelege werden im DTINT-Verfahren abgewickelt.  
 |
| **Alle folgenden Felder erscheinen nur bei aktiver Belegversand-Lizenz** |
| Versandprofil | Versandprofil aus dem [Versandprofilstamm](../../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/versandprofilstamm.md), welches zur Versendung dieser Belege verwendet werden soll. Wird hier nichts angegeben, so wird auch keine Avise versendet.  
 |
| Formular | Man kann hier ein vom Standard abweichendes Formular vom Typ Avise (Formulartyp=290) angeben um die Versandausgabe anders als die Druckausgabe zu formatieren. In der F3-Auswahl werden nur Formulare angeboten, bei denen die Archivierung aktiviert ist. Ist hier kein Formular angegeben, so wird dasselbe Formular wie beim Druck verwendet.  
 |
| Formular Mailbody | Avisen werden immer als Anhang an eine Mail versendet. Um die Mail selber zu gestalten, kann dafür hier ein Formular vom Typ Avise (Formulartyp=290) hinterlegt werden. Hier kann zusätzlich der Formularbereich „Avise Mail Betreffzeile“ für die Betreffzeile eingerichtet werden. Dieser Bereich wird nur beim Mailversand ausgewertet.  
Ein Beispielformular ist unter der Nummer -1100 zu finden.  
**HINWEIS:** *Um Grafiken in das Formular mit einzubinden, kann man den bekannten HTML-Syntax &lt;img src="cid:XXXXXX" alt="mein bild" /> verwenden. Für XXXXXX muss die GUID aus dem Formulararchiv, in dem die Grafik hinterlegt sein muss, angegeben werden.*  
 |
| Anstelle des Formulars für den Mailbody können auch Datenbankprozeduren verwendet werden. Ist in dem Feld „DB-Funktion Mailbody“ etwas eingetragen, dann werden die Prozeduren verwendet und das „Formular Mailbody“ wird ignoriert |
| FA-Eintrag Mailbody | FA-ID des Formulararchiv-Eintrags eines Mailbody-Templates, das mit Hilfe der Body-Funktion zu einem Mailbody verarbeitet werden kann.  
Der Eintrag hier ist optional, jedoch muss, wenn kein Template verwendet werden soll, die DB-Funktion das Dokument komplett aufbauen.  
 |
| DB-Funktion Mailbody | Wird hier eine Funktion hinterlegt, dann wird das „**Formular Mailbody**“ ignoriert und die Funktion liefert den Text der Mail. Sie erhält als Parameter die Zahlungsid aus der Tabelle ZAHLUNGSBELEG. Die Funktion muss einen Wert vom Typ „long varchar“ zurückgeben.  
Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.  
   
   
Die Mailbody Funktion sollte in jedem Fall eventuell auftretende Fehler abfangen und ins Fehlerprotokoll schreiben und anschließend **resignal** ausführen, weil ansonsten das verarbeitende Programm nicht mitbekommt, ob ein Fehler aufgetreten ist.  
 |
| DB-Funktion Betreff | Diese Funktion wird nur ausgeführt, wenn auch die Funktion für den Mailbody angegeben wurde. Es existiert eine einfache Funktion AMIC_BELEGVERSAND_BETREFF_AVISE, die Verwendet werden kann. Gibt man einen Namen einer nichtexistierenden Funktion an, um eine neue Funktion zu erstellen, dann wird die Funktion AMIC_BELEGVERSAND_BETREFF_AVISE als Vorlage verwendet und man kann seine Erweiterungen einbauen.  
   
   
Tritt hier ein Fehler auf und ist das Ergebnis leer, wird der Festtext Avise als Betreff eingetragen. |

**Organisatorisch sollen die Zahlungsarten für eRechnung so angelegt werden, dass diese nicht mit FIBU-Zahlungsarten verwechselt werden können.**
