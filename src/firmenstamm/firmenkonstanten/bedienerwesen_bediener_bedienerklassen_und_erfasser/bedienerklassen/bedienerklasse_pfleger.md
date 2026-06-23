# Bedienerklasse: Pfleger

<!-- source: https://amic.de/hilfe/_bedienerklassepflege.htm -->

<p class="just-emphasize">Register:</p>

<details>
<summary>Allgemein</summary>

| Felder | Beschreibung |
| --- | --- |
| Bedienerklasse | Eindeutige numerische Identifikation der Bedienerklasse<br> |
| Bezeichnung | Bezeichnung der Bedienerklasse<br> |
| Betriebsstätte | Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.<br>Standard: 0, ohne Filiale<br>Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden.<br> |
| Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte<br> |
| Login-Sperre | JA/NEIN<br>Login-Sperre aller Bediener dieser Bedienerklasse<br> |
| Formular FiBu-Infofenster | JA/NEIN<br>[Zusätzliche Informationen in der Finanzbuchhaltung aktivieren](../../../../finanzbuchhaltung/op_verwaltung/formular_fibu_infofenster.md)<br><br> |
| Toolbar aktiv | JA/NEIN<br>Aktiviert die standardmäßig ausgelieferte Toolbar.<br> |
| Abteilung | Hier kann die Bedienerklasse einer Abteilung zugeordnet werden.<br>[Abteilungen](../../../abteilungen.md)<br> |
| Controllerklasse | [Controllerklasse](./standard_bedienerklassen.md)<br>Controllerklassen werden gelb hervorgehoben.<br> |
| Sicherheitsklasse | [Sicherheitsklasse](./standard_bedienerklassen.md)<br> |

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
| Mindestlänge | Wie viele Zeichen das Passwort mindestens haben soll.<br> |
| Höchstlänge | Wie viele Zeichen das Passwort maximal haben darf.<br>Das Limit liegt bei 10 Zeichen.<br> |
| Zahlen | Wie viele Zeichen im Passwort mindestens eine Zahl (0-9) sein müssen.<br> |
| Sonderzeichen | Wie viele Zeichen im Passwort mindestens ein Sonderzeichen sein müssen.<br>Ausgenommen sind folgende Zeichen: „ \\ ; - ‘<br> |
| Aktualisierung in Tagen | Nach wie vielen Tagen seit der letzten Passwortänderung das Passwort wieder geändert werden muss.<br> |

    
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
| Speichern unter … Shift+F9 | Übernimmt die Daten der ausgewählten Bedienerklasse.<br>Das System schlägt eine neue Bedienerklasse vor.<br>Die Einstellungen hinsichtlich des Formulararchivs sind in diesem Modus nicht zu beeinflussen, die Registerkarte „Formulararchiv“ wird ausgeblendet.<br>Bei Ausführung der Funktion <strong>*Speichern*</strong> <strong>F9</strong> wird die neue Bedienerklasse zur weiteren Verwendung im System vorbereitet (der Vorgang kann etwas dauern und sollte nicht abgebrochen werden):<br>**Es werden dabei u.a. sämtliche Schutzeinstellungen der Funktionen (**[Zugriffsrechte Funktionen](../../zugriffsrechte_funktionen.md)**) der Ursprungsbedienerklasse übernommen.**<br>Ferner werden dabei alle Einstellungen der Ursprungsbedienerklasse „geclont“.<br>Bei den Formulararchiv-Einstellungen sind die Belege der Ursprungsklasse erlaubt, in den Einstellungen der Ursprungsklasse sind die Einstellungen der neuen Bedienerklasse erlaubt. |
| Löschen **F7** | Bedingung: Die Bedienerklasse darf keine Bediener mehr enthalten.<br>Alle Einstellungen der Bedienerklasse in den abhängigen Einstellungen ([Zugriffsrechte Funktionen](../../zugriffsrechte_funktionen.md), EPA, …) werden herausgenommen.<br>Zum jetzigen Zeitpunkt betrifft dies folgende Tabellen:<br>• NUMKREISBEDIENER<br>• KORRKLASSBEDLINK<br>• BEDIENERPROFIL<br>• ABTEILBEDIENLINK<br>• OWAAGENUMKREIS<br>• FIBUBELEGNUMKR<br>• BEDIENERGRUPLINK<br>• USERFELDER<br>• KONTRKLNUMKREIS<br>• FORMULARARCHIVBEDIENER<br>• AMIC_NEWS<br>• FA_IMPORT<br>• FA_IMPORT_REGEL<br>• FORMULARARCHIV_MANAGER<br>• AMIC_MENU_MESSAGES<br>• FA_VIEW_PROFIL<br>• FA_VIEW_PROFIL_DETAIL<br>• ANWENDFUNKKLASSE<br>• ANWENDOBKLASSE<br>• ANWENDVARKLASSE<br>• AMICINFOSYSMSKZUORD<br>• ROLLENKLASSE<br> |
| Neu **F8** | Neue Bedienerklasse anlegen.<br>Siehe aber auch ["Speichern unter ... SF9"](./bedienerklasse_pfleger.md#Speichern_unter_SF9) für die Anlage einer neuen Bedienerklasse mit gleichzeitiger Übernahme der Bedienerklassen-relevanten Einstellungen! |
| EPAs zeigen **F10** | Individuelle Steuerungen von Abläufen können in Anwendungen über Einrichterparameter (EPA) vorgenommen werden.<br>Diese Funktion ruft die entsprechende Anwendung zur Ansicht und Pflege der Einrichterparameter auf.<br>Hauptmenü > Administration > Steuerung > EPAs zeigen<br>Direktsprung **[EPAZ]** |
| Zugriffsrechte übertragen… | |
| NKZ-Zuordnung | Nummernkreiszuordnung der ausgewählten Bedienerklasse |
| Betriebsstätte | Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.<br>Standard: 0, ohne Filiale<br>Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden. |
| Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte |

</details>
