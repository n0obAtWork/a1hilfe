# Variante „Parameter-Übersicht“

<!-- source: https://amic.de/hilfe/_a1para.htm -->

Hauptmenü > Administration > Werkzeuge > Parameter-Übersicht

Direktsprung [PARA]

Mit Hilfe dieser Variante lassen sich die aktuellen Laufzeit-Parameter des A.eins-Clienten anzeigen.

Parameter haben diverse Quellen.  
    

- Einige werden beim Programmstart ermittelt.
- Andere Parameter werden ermittelt entsprechende Programmabschnitte durchlaufen werden.

Sie werden programmseitig oft mit einem Default-Wert vorbelegt, somit muss nicht jeder Parameter explizit angegeben werden.

So ermittelte Parameter werden gecached.

| Felder | Auswahlliste |
| --- | --- |
| Parameter | Der Name des Parameters |
| Wert | Der Wert des Parameters |
| Zugriffe | Gibt die Anzahl der Ermittlungen des Parameters innerhalb der aktuellen Laufzeit vom Programm an.<br>Hinweis: Da die Parameter zwischengespeichert sind erfolgt nicht jedes Mal ein erneuter Zugriff auf externe Ressourcen. |
| HERKUNFT | <pre><code>Gibt an wo das Parameter den Wert für den Parameter&#10; gefunden hat:&#10;DEFAULT: Es wurde keine explizite Parameterangabe&#10; gefunden und der übergebene Default-Wert genommen.&#10;CMDLINE: Parameter&#10; wurde per Kommandozeile übergeben.&#10;CALCULATED: Der Wert&#10; des Parameters wurde software-technisch ermittelt.&#10;HKCU-MANDANT:&#10; Windows-Registrierung&#10;AHOI-MANDANT: Mandant-Eintrag in&#10; `ahoi.ini`&#10;Weitere können&#10; sein:&#10; &#10; PH_CMDLINE = &#10; 0,&#10; &#10; PH_HKLM_MANDANT = 1,&#10; &#10; PH_HKLM_AMIC = &#10; 2,&#10; &#10; PH_AHOI_MANDANT = 3,&#10; &#10; PH_AHOI_AMIC = &#10; 4,&#10; &#10; PH_AMICCONF_MANDANT = 5,&#10; &#10; PH_AMICCONF_AMIC = 6,&#10; &#10; PH_HKCU_MANDANT = 7,&#10; &#10; PH_HKCU_AMIC = &#10; 8,&#10; &#10; PH_DEFAULT = &#10; 9,&#10; &#10; PH_DEFAULTSTARTUP = 10,&#10; &#10; PH_CALCULATED = 11,&#10; &#10; PH_DATABASECONNECT = 12,&#10; &#10; PH_UNBEKANNT &#10; = 99&#10;&#10;(99 ist kein Fehler sondern bedeutet das der Parameter nicht&#10; aus den vorherigen Quellen stammt.)</code></pre> |
| Nr | Laufende Nummerierung |

| Auswahlbedingungen | |
| --- | --- |
| Suchen | Führt eine Like-Suche in den Feldern Name und Wert durch |

| Funktionen | |
| --- | --- |
| Keine | |
