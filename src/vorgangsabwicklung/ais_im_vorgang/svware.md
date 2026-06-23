# SVWARE

<!-- source: https://amic.de/hilfe/svware.htm -->

Auf der SVWARE Maske wird nach jeder Eingabe eines Wertes und nach der Kontraktabwahl das zu aktualisieren AIS aufgerufen.

Folgende Nummer, IDs werden in Abhängigkeit des Feldes an das Makro übergeben. Ist der Typ eine Funktion wie z.B. die Kontraktabwahl, so konnte dem Aufruf zum aktualisieren des AIS kein eindeutiges Feld auf der SVWARE Maske zugeordnet werden.

| Maskenfeld | Übergebene IDs | Nummer | Typ |
| --- | --- | --- | --- |
| LagerNummer$ | ID_LAGERNUMMER | 1008 | Maskenfeld |
| ArtikelNummer$ | ID_ARTIKELNUMMER | 1006 | Maskenfeld |
| LagerPlatzNummer$ | ID_LAGERPLATZ | 1103 | Maskenfeld |
| Menge$ | ID_MENGE | 1001 | Maskenfeld |
| ME_Nummer$ | ID_ME_NUMMER | 1108 | Maskenfeld |
| FremdKtrNummer$ | ID_KTRNUMMER | 3003 | Maskenfeld |
| KTR.KtrNummer$ | ID_KTRNUMMER | 3003 | Maskenfeld |
| Preis$ | ID_PREIS | 1000 | Maskenfeld |
| PreisEinh$ | ID_PREISEINHEIT | 1078 | Maskenfeld |
| ME_NummerPreis$ | ID_ME_NUMMERPREIS | 1077 | Maskenfeld |
| Netto$ | ID_BETRAG | 1464 | Maskenfeld |
| SkontoKennz$ | ID_SKONTIERFAEHIG | 1030 | Maskenfeld |
| ZusatzInfo$ | ID_ZUSATZINFO | 1353 | Maskenfeld |
| ZusatzInfo2$ | ID_ZUSATZINFO2 | 1358 | Maskenfeld |
| LiefDat$ | ID_LIEFDAT | 1009 | Maskenfeld |
| Liefernummer$ | ID_LIEFNUMMER | 1042 | Maskenfeld |
| FiktivMenge$ | ID_ME_NUMMER_FIKTIV | 1825 | Maskenfeld |
| KolloMEErgebnis$ | ID_PREISBEZUG | 540 | Maskenfeld |
| GeschaeftsArt$ | ID_GESCHAEFTSART | 458 | Maskenfeld |
| artirab$ | ID_ZUABNUMMER | 1343 | Maskenfeld |
| rabsatz$ | ID_ZUABPREIS | 1242 | Maskenfeld |
| rabnetto$ | ID_BETRAGZUAB | 1465 | Maskenfeld |
| PartieGrid | ID_PARTIENUMMER | 1473 | Funktion |
| Kontraktabwahl | ID_KTRNUMMER | 3003 | Funktion |
| Nachhaltig | ID_NH_STATUS | 7050 | Funktion |

<p class="just-emphasize">Benötigte JVARS</p>

| JAVR | Funktion | Bedeutung |
| --- | --- | --- |
| VORGANGHANDLE | Lesend | Mit dieser JAVR wird der aktuelle Handle des Vorgangs übergeben |
| WAPOSITIONHANDLE | Lesend<br> | Mit dieser JVAR wird der Handle der Warenposition übergeben. |
| ID | Lesend<br> | Mit dieser JVAR wird die Nummer der ID übergeben |
| FELDNAME | Lesend | Diese JVAR enthält den Namen des aufrufenden Feldes. Das Feld kann aber auch eine Funktion sein. Es wird der Feldname aus der Spalte Maskenfeld von der Tabelle drüber an das Makro übermittelt. |

<p class="just-emphasize">Makro Beispiel</p>

```text
CONST
  BUFLEN = 256;
  ID_PARTIENUMMER = 1473;
PROCEDURE TesteSVWARE();
  var

lAus          :string;
    lsVorHandle   :string;
    lsWaposHandle :string;
    lParameter
:string;
    lFeldName
:string;

lBuf          :string;
    lVorgHandle
:integer;
    lWaposHandle  :integer;

lStrId        :integer;

lFeldId       :integer;
BEGIN

lAus          := alloc(BUFLEN);

lBuf          := alloc(BUFLEN);
    lsVorHandle   :=
alloc(BUFLEN);
    lsWaposHandle := alloc(BUFLEN);
    lParameter    :=
alloc(BUFLEN);
    lFeldName     :=
alloc(BUFLEN);
    strcpy(lAus, "");
    strcpy(lsVorHandle, "");
    strcpy(lsWaposHandle, "");
    bagget("VORGANGHANDLE", lsVorHandle,
BUFLEN);
    bagget("WAPOSITIONHANDLE",
lsWaposHandle, BUFLEN);
    bagget("ID", lParameter,
BUFLEN);
    bagget("FELDNAME", lFeldName,
BUFLEN);
    StrRTrim(lFeldName);
    lWaposHandle :=
strtoint(lsWaposHandle);

lFeldId      := strtoint(lParameter);
    lStrId  :=
StrCmp(lFeldName,  "PartieGrid" );
    if ( lStrId = 0 ) then
    Begin
      strcpy(lAus, "");
      GetValPos(lWaposHandle,
ID_PARTIENUMMER, lBuf,0);
      sprintf(lAus,"Hier kommt
meine UNIT TesteSVWARE Feldname <%s> Wert der Partienummer <%s>",
lFeldName, lBuf);
      MessageBox( lAus , "Unit
Test" , 1 );
      dbx_io
("AISREFRESH","Zeit$", "", "") ;
    End;
    free(lAus);
    free(lBuf);
    free(lsVorHandle);
    free(lsWaposHandle);
    free(lParameter);
    free(lFeldName);
END;
```

In diesem Beispiel wird geprüft, ob das auslösende Feld, welches das Makro aufruft, die Partie ist. Wenn dies der Fall ist, wird das AIS aktualisiert. Bei jedem anderen Aufruf des Makros wird das AIS nicht aktualisiert.

Bei einem Vergleich des Feldnamen mit einem string (wie in obigen Beispiel), muss dies über die StrCmp Funktion passieren. Wenn der Feldname gleich dem zu vergleichendem Text ist, so liefert die Funktion eine 0 zurück.

```text
lStrId  :=
StrCmp(lFeldName,  "PartieGrid" );
if ( lStrId = 0 )
then
```
