# Die Anwendung “JVARS”

<!-- source: https://amic.de/hilfe/_jvars_aw.htm -->

Administration > Werkzeuge

oder Direktsprung [**JVARS**]

JVARS sind zur Laufzeit existierende Speicherinhalte, die somit nur auf dem aktiven Client existieren und erst einmal per Se keinen Datenbankbezug haben.

| Felder | Beschreibung |
| --- | --- |
| Bereich | Die Owner von JVARS sind in folgende Bereiche aufgeteilt:<br>**Papierkorb**(0): Spezial-JVar. Sie wird vom Basissystem für kurzfristige Speicherungen verwendet. Eine eigene Verwendung ist nicht vorgesehen!<br>**AMIC**(1 bis 9999): AMIC-JVars, **Schreibzugriffe sollten vermieden werden**<br>[Beschreibung einiger ausgewählter JVARS](./ausgewaehlte_jvars.md)<br>**Anwender/Support**(10000 bis 19999): Private JVARS: Diese JVars werden vom Programm nicht verwendet. Diese JVARS können eigenverantwortlich gewählt und verwendet werden.<br>**Programm/Laufzeit**(ab 20000): normale JVars zur Laufzeit, also solchen die die API-Funktionen zur Gewinnung einer JVAR verwenden. |
| Owner | Eine ganzzahlige Zahl >= 0<br>Jeder Laufzeit-Instanz einer Maske wird ein Owner zugeordnet.<br>Der Owner der aktiv war zum Zeitpunkt des Aufrufs dieser Anwendung JVARS wird grün dargestellt. |
| JVar | Ein alphanumerischer Schlüsselbegriff der eine JVAR bezeichnet, innerhalb eines Owners sind diese eindeutig!<br>Die JVARS des Owners 3561 werden gelb dargestellt. |
| Wert | Ein alphanumerischer Wert |

| Felder | Beschreibung |
| --- | --- |
| Owner | Numerisch, von – bis |
| Name | Like |
| Wert | Like |

| Felder | Beschreibung |
| --- | --- |
| hexadezimale Debuganzeige | Erlaubt die hexadezimale Speicher-Einsicht eines JVAR-Wertes<br>Beispiel:<br>73 65 6C 65 63 74 20 6F 62 2E 61 6E 77 66 75 6E select ob.anwfun<br>6B 69 64 2C 20 6F 62 2E 6F 70 74 62 6F 78 69 64 kid, ob.optboxid<br>2C 20 69 66 20 27 4F 42 5F 6A 76 61 72 73 27 20 , if 'OB_jvars'<br>3D 20 6F 62 2E 6F 70 … |
| Stack-Anzeige | Jede JVAR ist intern als Stack aufgebaut, kann also mehrere Werte verwalten.<br>Beispiel für eine JVAR mit mehr als einem Eintrag im Stack:<br>&lt;stackdump owner="6000" key="K_MASKE" size="2"><br>&lt;Entry nr="1">AW_MASK&lt;/Entry><br>&lt;Entry nr="2">A1NETMENU&lt;/Entry><br>&lt;/stackdump> |
