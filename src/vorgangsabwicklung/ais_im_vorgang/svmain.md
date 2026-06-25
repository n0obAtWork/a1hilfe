# SVMAIN

<!-- source: https://amic.de/hilfe/svmain.htm -->

Das AIS wird auf der SVMAIN immer nach der Eingabe des Kunden komplett aktualisiert, die Aktualisierung des AIS in Abhängigkeit der UFLD-Felder erfolgt per Makro und wird nur dann aufgerufen, wenn ein Wert in dem UFLD-Feld geändert worden ist.

Des Weiteren kann das AIS nach der Eingabe einer manuellen Adresse oder einer manuellen Versandadresse aktualisiert werden. Beim Ändern der Werte in der allgemeinen Vorgangszuordnung wird das AIS auf der SVMAIN Maske aktualisiert.

<p class="just-emphasize">Aktualisierungspunkte des AIS auf der SVMAIN Maske</p>

| Feld | Aktion |
| --- | --- |
| Kundennummer | Immer |
| Rückkehr aus der Posbar2 | Immer |
| Unterklassenänderung | Immer |
| Klassenänderung | Immer |
| UFLD | Kann per Makro gesteuert werden. Es gibt aber Ausnahmen, dort wird schon im Standard eine Aktualisierung des kompletten AIS vorgenommen. Es existiert aber die Möglichkeit, die komplette Aktualisierung des AIS im Makro zu verhindern. |
| Änderung der Adresse | Das Aktualisieren des AIS kann per Makro gesteuert werden. Die JVAR UFLDID wird nicht gesetzt. |
| Änderung der Versandadresse | Das Aktualisieren des AIS kann per Makro gesteuert werden. Die JVAR UFLDID wird nicht gesetzt |
| Vorgang Zuordnung | Das Aktualisieren des AIS kann per Makro gesteuert werden. Die JVAR UFLDID wird nicht gesetzt. In der Feld ID wird die ID des Feldes übergeben. |

<p class="just-emphasize">Folgende JVARS werden an das Makro Übermittelt</p>

| JAVR | Funktion | Bedeutung |
| --- | --- | --- |
| UFLDID | Lesend | Mit dieser JAVR wird die UFLD ID des Feldes übergeben, welches geändert worden ist. Dieses Feld ist leer, wenn eine Aktualisierung der manuellen Versandadresse vorgenommen wird. |
| VORGANGHANDLE | Lesend | Mit dieser JVAR wird der Vorgangshandel des aktiven Vorgangs übergeben. |
| GLOBALREFRESH | Schreibend<br> | Diese JAVR kann aus dem Makro gesetzt werden, damit die Globale AIS Aktualisierung ausgeschaltet werden kann, wenn das UFLD-Feld den Wert „Update Mask“ auf ja stehen hat. Im Standard steht der Wert dieser JAVR auf 0<br><ul><li>&nbsp;&nbsp;&nbsp; 0 bedeutet Global Refresh</li><li>&nbsp;&nbsp;&nbsp; 1 bedeutet Eigenes Refresh</li></ul> |
| ID | Lesend | Ist der Wert der JVAR größer als 0, enthält die JVAR eine ID des Vorgangs, wie z.B. Die ID_VERSANDADRESSID |

<p class="just-emphasize">Folgende User Felder lösen ein Globales Aktualisieren des AIS aus</p>

| ID | Nummer | UFLD Bezeichnung |
| --- | --- | --- |
| ID_VERSANDADRESSID | 108 | Versandadresse |
| ID_INFOADRESSID | 547 | informelle Anschrift |
| ID_VERSANDARTID | 1034 | Versandart |
| ID_VERKAUFSGEBIET | 1096 | Verkaufs Gebiet |
| ID_LKW_NUMMER_MOTOR | 1099 | LKW Nummer Motor |
| ID_LKW_NUMMER_ANHAENGER | 1100 | LKW Anhänger |
| ID_REFERENZNUMMER | 1205 | Referenznummer |
| ID_FW_NUMMER | 1453 | Währungsnummer |
| ID_LAGERNUMMER_FEHL | 1510 | Lagernummer. |
| ID_PARITAETNUMMER | 1828 | Parität |
| ID_AKTOBJEKT_NUMMER | 1851 | Objekte |
| ID_KUNDNUMMER_RECHNUNGSEMPF | 1894 | Rechnungsempfänger |
| ID_KUNDNUMMER_ZAHLUNGSPFL | 1895 | Zahlungspflichtiger |
| ID_KUNDNUMMER_KONTRAKTKUNDE | 1899 | Kontraktkunde |

<p class="just-emphasize">Übermitteltete Werte wenn die JVAR ID gesetzt worden ist.</p>

| Maskenfeld | ID | Nummer | Event |
| --- | --- | --- | --- |
| Kein Maskenfeld | ID_VERSANDADRESSID | 108 | Änderung der Versandadresse /<br>Änderung der Adresse |
| Kein Maskenfeld | ID_KUNDNUMMER | 106 | Änderung der Versandadresse /<br>Änderung der Adresse |
| FraKlassNummer$ | ID_FRAKLASSNUMMER | 1067 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| FraVariNummer$ | ID_FRACHTVARIANTE | 1095 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| VersArtId$ | ID_VERSANDARTID | 1034 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| AdressNummer$ | ID_VERSANDADRESSID | 108 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| GebietNummerVon$ | ID_GEBIET_VON | 1097 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| GebietNummerNach$ | ID_GEBIET_NACH | 1098 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| LKW_NummerMotor$ | ID_LKW_NUMMER_MOTOR | 1099 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| LKW_NummerAnhaen$ | ID_LKW_NUMMER_ANHAENGER | 1100 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| FahrerNummer$ | ID_FAHRER | 1101 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| VerkGebNummer$ | ID_VERKAUFSGEBIET | 1096 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |
| VertGrNummer$ | ID_VERTGRNUMMER | 1062 | Wenn das Ändern des Wertes funktioniert hat. Wird das AIS Refresh aufgerufen |

<p class="just-emphasize">Beispiel Prozedur für die SVMAIN</p>

```text
const BUFLEN = 256;
procedure TestSVMAIN();
  var
    lAus : string;
    lVHandle : integer;
    lUFLDID  : integer;
    lBuf: string;
    lBuf1: string;
    lBuf2: string;
BEGIN
    lBuf  := alloc(BUFLEN);
    lBuf1 := alloc(BUFLEN);
    lBuf2 := alloc(BUFLEN);
    lAus  := alloc(BUFLEN);
    bagget("UFLDID", lBuf1, BUFLEN);
    bagget("VORGANGHANDLE", lBuf2, BUFLEN);
    lVHandle := strtoint(lBuf2);
    lUFLDID  := strtoint(lBuf1);
    GetValue(lVHandle, lUFLDID, lAus,0);
    if (lUFLDID = 1772 ) then
    Begin
      dbx_io ("AISREFRESH","Zeit$", "", "") ;
    End;
    if (lUFLDID = 1034 ) then
    Begin
      bagset("GLOBALREFRESH","0");
    End;
    free(lAus);
    free(lBuf);
    free(lBuf1);
    free(lBuf2);
END;
```

In diesem Beispiel wird das AIS-Feld aktualisiert, wenn das UFLD-Feld mit der ID 1772 geändert worden ist. Des Weiteren wurde das Aktualisieren des kompletten AIS ausgeschaltet, wenn das UFLD Feld mit der ID 1034 geändert wird.
