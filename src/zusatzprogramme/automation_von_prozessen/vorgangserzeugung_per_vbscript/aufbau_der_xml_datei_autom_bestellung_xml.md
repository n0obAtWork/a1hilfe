# Aufbau der XML Datei „autom_bestellung.xml“

<!-- source: https://amic.de/hilfe/aufbauderxmldateiautombestellu.htm -->

Die Datei gliedert sich zurzeit in die folgenden drei Abschnitte

```text
<neuerVorgang>

<positionHinzufuegen>

<positionZusammenfuehren>
```

Die Bedeutung der Abschnitte ist selbsterklärend. In diesen Abschnitten werden die einzelnen Vorgangsattribute zu den jeweiligen Vorgängen definiert. Obwohl eine Überprüfung der hier eingegebenen Daten im lesenden Script (bestellung_start.vbs) durchgeführt wird, ist eine sorgsame Dateneingabe für die automatische Durchführung der Vorgänge notwendig. So sollte darauf geachtet werden das die eingegebenen Daten (wie z.B. Kundenummer, Partie, Artikel etc.) im System enthalten sind und sie korrekt eingegeben werden.

Zu beachten sind die folgenden Attribute

```text
<d_artikelid>

<d_partieid>

<s_artikelid>

<s_partieid>
```

Hier muss entgegen der Attributbezeichnung nicht die ID sondern die jeweilige Nummer eingegeben werden.

Außerdem müssen Mengen mit einem Dezimalpunkt als Trennzeichen eingegeben werden.

Beispiel:

```xml
<neuerVorgang>

<bewegungsstatus>1</bewegungsstatus>

<d_artikelid>2100</d_artikelid>

<d_kundnummer>300005</d_kundnummer>

<d_v_klassnummer>1400</d_v_klassnummer>

<prozessid>3780</prozessid>

<s_artikelid>2100</s_artikelid>

<s_jahrnummer>2007</s_jahrnummer>

<s_menge>8.88</s_menge>

<s_partieid>95</s_partieid>

<s_v_klassnummer>400</s_v_klassnummer>

<wer>HA</wer>
    ...

</neuerVorgang>

<positionHinzufuegen>

<bewegungsstatus>1</bewegungsstatus>

<d_artikelid>2100</d_artikelid>

<d_jahrnummer>2007</d_jahrnummer>

<d_kundnummer>300005</d_kundnummer>

<d_v_klassnummer>1400</d_v_klassnummer>

<prozessid>3780</prozessid>

<s_artikelid>2100</s_artikelid>

<s_jahrnummer>2007</s_jahrnummer>

<s_menge>3.33</s_menge>

<s_partieid>97</s_partieid>

<s_v_klassnummer>400</s_v_klassnummer>

<wer>HA</wer>
    ...

</positionHinzufuegen>

<positionZusammenfuehren>

<bewegungsstatus>1</bewegungsstatus>

<d_artikelid>2100</d_artikelid>

<d_jahrnummer>2007</d_jahrnummer>

<d_kundnummer>300005</d_kundnummer>

<d_v_klassnummer>1400</d_v_klassnummer>

<prozessid>3780</prozessid>

<s_artikelid>2100</s_artikelid>

<s_jahrnummer>2007</s_jahrnummer>

<s_menge>4.44</s_menge>

<s_partieid>98</s_partieid>

<s_v_klassnummer>400</s_v_klassnummer>

<wer>HA</wer>

...
  </positionZusammenfuehren>
```
