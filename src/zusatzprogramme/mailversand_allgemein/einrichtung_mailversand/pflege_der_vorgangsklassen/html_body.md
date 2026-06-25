# HTML-Body

<!-- source: https://amic.de/hilfe/htmlbody.htm -->

E-Mails werden schon lange nicht mehr als reiner ASCII-Text versendet, sondern oftmals im HTML-Format. Damit der E-Mail-Body in HTML darstellen lässt und so die Auswahl von Schriftarten, Schriftstilen oder Schriftgrößen ebenso darstellen wie digitale Grafiken.

Damit Inhalte des Belegs in das HTML eingefügt werden können, erstellen Sie eine Datenbankfunktion, die aus dem zu versendenden Beleg Daten herauskristallisiert und in ein HTML einsetzt.

Das „ROH“-HTML wird als Eintrag ins Archiv gestellt und seine Eintrags-ID wird in der [Formulararchivzuordnung](../../../../vorgangsabwicklung/formularzuordnung/abwicklung.md#Abwicklung_Versand) hinterlegt.

Als Beispiel verwenden Sie dazu die Datenbankfunktion AMIC_DEMO_HTMLBODY

```text
---<class name="AMIC_DEMO_HTMLBODY"/>
---<summary>AMIC_DEMO_HTMLBODY</summary>
---<returns>HTMLBody</returns>
---<param name="in_fa_id">fa_id des zu versendenden Ware-Beleges</param>
---<param name="in_fa_mndNr">Mandantnummer des zu versendenden Ware-Beleges</param>
CREATE FUNCTION AMIC_DEMO_HTMLBODY(in_fa_id integer, in_fa_mndNr integer)
returns long varchar
```

1. Schritt – Erstellen einer HTML-Body-Datei

Erstellen Sie eine HTML-Datei, die den Inhalt des Mailtextes darstellen soll. Verwenden Sie dazu die HTML-Tags in spitzen Klammern.

Wollen Sie im Text Inhalte aus dem zu versendenden Beleg Informationen einsetzen, so müssen Sie dafür einen Platzhalter zu definieren. In unserem Beispiel wird ein Wort in geschweiften Klammern verwendet. Das ist aber nicht zwingend so.

Verboten für die Kennzeichnung von Platzhaltern sind wegen der Verwechslungsgefahr mit HTML-Tags die spitze Klammer.

Siehe auch Service einrichten für den A.eins Mailversand

2. Schritt – Importieren der HTML-Body-Datei

Importieren Sie ins Formulararchiv diese Datei unter Angabe der Belegklasse 8039 – HTMLBODY. Bitte geben Sie einen Betreff an, den Sie später im FRZ wieder finden.

Merken Sie sich die FA_ID dieses Eintrags.

3. Schritt – Definieren einer HTML-Body-Funktion

Damit die Platzhalter für die Inhalte nun gegen Inhalte ausgetauscht werden können, wird eine Datenbankfunktion verwendet. In dieser wird der Text der in Schritt 1 erstellten Datei aus der Datenbank geladen. Zusätzlich werden Daten aus den Angaben des zu vermailenden Belegs ermittelt und die können dann die Platzhalter ersetzen.

4. Schritt – Eintrag in die Formularzuordnung

Tragen Sie in der Formularzuordnung die FA_ID der HTML-Body-Datei und die HTML-Body-Funktion ein.

Siehe auch [Formularzuordnung **[FRZ]**](../../../../vorgangsabwicklung/formularzuordnung/abwicklung.md#Abwicklung_Versand)**.**
