# SVPOSI

<!-- source: https://amic.de/hilfe/svposi.htm -->

Die AIS-Aktualisierung findet nur dann statt, wenn eine Position ausgewählt oder ein Positionswechsel vollzogen wird. Das AIS wird beim Zurückkommen von einer darüberlegenden Maske noch einmal aktualisiert.

Alle AIS-Aktualisierungspunkte auf der SVPOSI Maske können per Makro gesteuert werden. Es gibt keine Punkte, die im Standard das ganze AIS auf der Maske aktualisiert wird.

#### Bei folgenden Ereignissen wird das AIS aktualisiert.

1. Navigieren nach oben

2. Navigieren nach unten

3. Sprung zum Positionsanfang

4. Sprung zum Positionsende

5. Anwählen einer Positionszeile mit der Maus

6. Rückkehr von einer darüber liegenden Maske auf die SVPOSI Maske

#### Benötigte JVARS

| JAVR | Funktion | Bedeutung |
| --- | --- | --- |
| VORGANGHANDLE | Lesend<br> | Mit dieser JVAR wird der Vorgangshandel des aktiven Vorgangs übergeben. |
| POSITIONHANDLE | Lesend<br> | Mit dieser JVAR wird das aktuelle Positionshandle übergeben. Über den Positionshandle kann der Typ der Positionszeile bestimmt werden. |
| PARAM | Lesend<br> | Diese JVAR ist nicht immer versorgt. Enthält diese JVAR ein Wert, so ist es der Zeilentyp. |

#### Beispiel Prozedur des Makro für die SVPOSI

```text
PROCEDURE TesteSVPOSI();
  var
    lAus : string;
    lBuf : string;
    lsVorHandle :string;
    lsWaposHandle :string;
    lParameter    :string;
    lVorgHandle :integer;
    lPosHandle : integer;
    lPosTyp    : integer;
BEGIN
    lAus  := alloc(BUFLEN);
    lBuf  := alloc(BUFLEN);
    lsVorHandle := alloc(BUFLEN);
    lsPosHandle := alloc(BUFLEN);
    lParameter := alloc(BUFLEN);
    strcpy(lAus, "");
    strcpy(lsVorHandle, "");
    strcpy(lsWaposHandle, "");
    strcpy(lParameter, "");
    strcpy(lBuf, "");
    bagget("VORGANGHANDLE", lsVorHandle, BUFLEN);
    bagget("POSITIONHANDLE", lsPosHandle, BUFLEN);
    bagget("PARAMETER", lParameter, BUFLEN );
    lPosHandle := strtoint(lsPosHandle);
    lPosTyp := GetPosType(lPosHandle);
    if ( lPosTyp  = 101 ) then
    Begin
      dbx_io ("AISREFRESH","Zeit$", "", "");
    End;
    free(lsVorHandle);
    free(lsWaposHandle);
    free(lParameter);
    free(lAus);
END;
```

In diesem Beispiel wird das AIS-Feld Zeit$ auf der SVPOSI Maske aktualisiert, wenn meine ausgewählte Zeile eine Warenpositionszeile ist.
