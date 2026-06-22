# Betrag in Worten drucken

<!-- source: https://amic.de/hilfe/betraginwortendrucken.htm -->

Beim Druck von Formularen können bisher Beträge auch in Worten ausgedruckt werden. Dafür muss in den Details des Formulareinrichters (Direktsprung FRM, Formulareinrichtung F6, Einrichtung F6 und dann der Knopf „Detail“) der Schalter „In Worten darstellen“ angewählt werden. Dadurch werden dann die einzelnen Ziffern des Betrages (ohne Nachkomma) in Wortdarstellung gedruckt, z.B. „eins-acht-vier-sieben“.

Bei dieser Darstellung wird derzeit auch die intern gewählte Systemsprache berücksichtigt. Es lassen sich also die Ziffern in die entsprechende Systemsprache übersetzen.

Für den Fall, dass jedoch der Betragtext in unterschiedlicher Sprache dargestellt werden soll, weil z.B. für ausländische Kunden sprachgerechte Formulare eingerichtet werden müssen, kann das bisherige Verfahren dafür nicht benutzt werden.

Daher wurde jetzt die Möglichkeit geschaffen, mit einer privaten Datenbankfunktion die Textaufbereitung eines Betrages selbst zu ‚programmieren’.

Zu diesem Zweck wurde im Formularstamm das Feld ‚[DB Fkt. Num Text](./db_fkt_num_text.md)’ hinzugefügt. Hier muss der Name einer privaten Datenbankfunktion eingetragen werden ([Parametererklärung siehe weiter unten](./parameter_der_datenbankfunktion.md)). Diese Datenbankfunktion erledigt dann die Textaufbereitung aller Betragspositionen dieses Formulars, bei denen der Detail-Schalter „In Worten darstellen“ angewählt wurde.
