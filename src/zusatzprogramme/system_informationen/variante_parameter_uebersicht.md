# Variante „Parameter-Übersicht“

<!-- source: https://amic.de/hilfe/_a1para.htm -->

Hauptmenü > Administration > Werkzeuge > Parameter-Übersicht

Direktsprung [PARA]

Mit Hilfe dieser Variante lassen sich die aktuellen Laufzeit-Parameter des A.eins-Clienten anzeigen.

Parameter haben diverse Quellen.  
    

\- Einige werden beim Programmstart ermittelt.

\- Andere Parameter werden ermittelt entsprechende Programmabschnitte durchlaufen werden.

Sie werden programmseitig oft mit einem Default-Wert vorbelegt, somit muss nicht jeder Parameter explizit angegeben werden.

So ermittelte Parameter werden gecached.

| Felder | Auswahlliste |
| --- | --- |
| Parameter | Der Name des Parameters |
| Wert | Der Wert des Parameters |
| Zugriffe | Gibt die Anzahl der Ermittlungen des Parameters innerhalb der aktuellen Laufzeit vom Programm an.  
Hinweis: Da die Parameter zwischengespeichert sind erfolgt nicht jedes Mal ein erneuter Zugriff auf externe Ressourcen. |
| HERKUNFT | 
```text
Gibt an wo das Parameter den Wert für den Parameter
      gefunden hat:
DEFAULT: Es wurde keine explizite Parameterangabe
      gefunden und der übergebene Default-Wert genommen.
CMDLINE: Parameter
      wurde per Kommandozeile übergeben.
CALCULATED: Der Wert
      des Parameters wurde software-technisch ermittelt.
HKCU-MANDANT:
      Windows-Registrierung
AHOI-MANDANT: Mandant-Eintrag in
      `ahoi.ini`
Weitere können
      sein:

      PH_CMDLINE          =
      0,

      PH_HKLM_MANDANT     =  1,

      PH_HKLM_AMIC        =
      2,

      PH_AHOI_MANDANT     =  3,

      PH_AHOI_AMIC        =
      4,

      PH_AMICCONF_MANDANT =  5,

      PH_AMICCONF_AMIC    =  6,

      PH_HKCU_MANDANT     =  7,

      PH_HKCU_AMIC        =
      8,

      PH_DEFAULT          =
      9,

      PH_DEFAULTSTARTUP   = 10,

      PH_CALCULATED       = 11,

      PH_DATABASECONNECT  = 12,

  PH_UNBEKANNT
      = 99

(99 ist kein Fehler sondern bedeutet das der Parameter nicht
      aus den vorherigen Quellen stammt.)
```

 |
| Nr | Laufende Nummerierung |

| Auswahlbedingungen | |
| --- | --- |
| Suchen | Führt eine Like-Suche in den Feldern Name und Wert durch |

| Funktionen | |
| --- | --- |
| Keine | |
