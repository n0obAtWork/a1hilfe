# Bedienerklasse: Pfleger

<!-- source: https://amic.de/hilfe/_bedienerklassepflege.htm -->

<p class="just-emphasize">Register:</p>

<details>
<summary>Allgemein</summary>

| Felder | Beschreibung |
| --- | --- |
| Bedienerklasse | Eindeutige numerische Identifikation der Bedienerklasse  
 |
| Bezeichnung | Bezeichnung der Bedienerklasse  
 |
| Betriebsstätte | Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.  
Standard: 0, ohne Filiale  
Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden.  
 |
| Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte  
 |
| Login-Sperre | JA/NEIN  
Login-Sperre aller Bediener dieser Bedienerklasse  
 |
| Formular FiBu-Infofenster | JA/NEIN  
[Zusätzliche Informationen in der Finanzbuchhaltung aktivieren](../../../../finanzbuchhaltung/op_verwaltung/formular_fibu_infofenster.md)  
    
 |
| Toolbar aktiv | JA/NEIN  
Aktiviert die standardmäßig ausgelieferte Toolbar.  
 |
| Abteilung | Hier kann die Bedienerklasse einer Abteilung zugeordnet werden.  
[Abteilungen](../../../abteilungen.md)  
 |
| Controllerklasse | [Controllerklasse](./standard_bedienerklassen.md)  
Controllerklassen werden gelb hervorgehoben.  
 |
| Sicherheitsklasse | [Sicherheitsklasse](./standard_bedienerklassen.md)  
 |

</details>

 

<details>
<summary>Formulararchiv</summary>

Hier kann für eine Bedienerklasse festgelegt werden, welche Formulare des Archivs (abhängig von den zugeordneten Bedienerklassen) eingesehen werden dürfen.

Diese Rechte können auch beim [Bediener](./standard_bedienerklassen.md) (Register Formulararchiv) eingesehen werden.

</details>

<details>
<summary>Passwortrichtlinien</summary>

| Felder | Beschreibung |
| --- | --- |
| Mindestlänge | Wie viele Zeichen das Passwort mindestens haben soll.  
 |
| Höchstlänge | Wie viele Zeichen das Passwort maximal haben darf.  
Das Limit liegt bei 10 Zeichen.  
 |
| Zahlen | Wie viele Zeichen im Passwort mindestens eine Zahl (0-9) sein müssen.  
 |
| Sonderzeichen | Wie viele Zeichen im Passwort mindestens ein Sonderzeichen sein müssen.  
Ausgenommen sind folgende Zeichen: „ \\ ; - ‘  
 |
| Aktualisierung in Tagen | Nach wie vielen Tagen seit der letzten Passwortänderung das Passwort wieder geändert werden muss.  
 |

    
**Die Passwortrichtlinien werden je nach Bedienerklasse individuell verwaltet und gepflegt.**

**Die Bearbeitung und Anpassung der Passwortrichtlinien ist ausschließlich Systemadministratoren vorbehalten.**

Zur Änderung des eigenen Passworts kann über den Direktsprung **[PWD]** die entsprechende Ansicht zur Passwortänderung geöffnet werden.

Wird ein Wert auf **0** gesetzt oder das Kontrollkästchen deaktiviert, so wird die jeweilige Bedingung bei der Prüfung nicht berücksichtigt.

Die Richtlinie **„Aktualisierung in Tagen“** wird nur beim Anmeldevorgang in **A.eins** überprüft.

<p class="just-emphasize">Funktion:</p>

</details>

<details>
<summary>Bedienerklasse: Pfleger Funktionen</summary>

Im Pfleger stehen folgende Funktionen zur Verfügung:

| Funktionen |
| --- |
| Speichern F9 | Speichert die Daten |
| Speichern unter … Shift+F9 | Übernimmt die Daten der ausgewählten Bedienerklasse.  
Das System schlägt eine neue Bedienerklasse vor.  
Die Einstellungen hinsichtlich des Formulararchivs sind in diesem Modus nicht zu beeinflussen, die Registerkarte „Formulararchiv“ wird ausgeblendet.  
Bei Ausführung der Funktion ***Speichern*** **F9** wird die neue Bedienerklasse zur weiteren Verwendung im System vorbereitet (der Vorgang kann etwas dauern und sollte nicht abgebrochen werden):  
**Es werden dabei u.a. sämtliche Schutzeinstellungen der Funktionen (**[Zugriffsrechte Funktionen](../../zugriffsrechte_funktionen.md)**) der Ursprungsbedienerklasse übernommen.**  
Ferner werden dabei alle Einstellungen der Ursprungsbedienerklasse „geclont“.  
Bei den Formulararchiv-Einstellungen sind die Belege der Ursprungsklasse erlaubt, in den Einstellungen der Ursprungsklasse sind die Einstellungen der neuen Bedienerklasse erlaubt. |
| Löschen **F7** | Bedingung: Die Bedienerklasse darf keine Bediener mehr enthalten.  
Alle Einstellungen der Bedienerklasse in den abhängigen Einstellungen ([Zugriffsrechte Funktionen](../../zugriffsrechte_funktionen.md), EPA, …) werden herausgenommen.  
Zum jetzigen Zeitpunkt betrifft dies folgende Tabellen:  
• NUMKREISBEDIENER  
• KORRKLASSBEDLINK  
• BEDIENERPROFIL  
• ABTEILBEDIENLINK  
• OWAAGENUMKREIS  
• FIBUBELEGNUMKR  
• BEDIENERGRUPLINK  
• USERFELDER  
• KONTRKLNUMKREIS  
• FORMULARARCHIVBEDIENER  
• AMIC_NEWS  
• FA_IMPORT  
• FA_IMPORT_REGEL  
• FORMULARARCHIV_MANAGER  
• AMIC_MENU_MESSAGES  
• FA_VIEW_PROFIL  
• FA_VIEW_PROFIL_DETAIL  
• ANWENDFUNKKLASSE  
• ANWENDOBKLASSE  
• ANWENDVARKLASSE  
• AMICINFOSYSMSKZUORD  
• ROLLENKLASSE  
 |
| Neu **F8** | Neue Bedienerklasse anlegen.  
Siehe aber auch ["Speichern unter ... SF9"](./bedienerklasse_pfleger.md#Speichern_unter_SF9) für die Anlage einer neuen Bedienerklasse mit gleichzeitiger Übernahme der Bedienerklassen-relevanten Einstellungen! |
| EPAs zeigen **F10** | Individuelle Steuerungen von Abläufen können in Anwendungen über Einrichterparameter (EPA) vorgenommen werden.  
Diese Funktion ruft die entsprechende Anwendung zur Ansicht und Pflege der Einrichterparameter auf.  
Hauptmenü > Administration > Steuerung > EPAs zeigen  
Direktsprung **[EPAZ]** |
| Zugriffsrechte übertragen… | |
| NKZ-Zuordnung | Nummernkreiszuordnung der ausgewählten Bedienerklasse |
| Betriebsstätte | Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.  
Standard: 0, ohne Filiale  
Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden. |
| Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte |

</details>
