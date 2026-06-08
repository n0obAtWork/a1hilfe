# Bedienerklasse: Pfleger

<p class="just-emphasize">Register:</p>

<details>
  <summary>Allgemein</summary>

  | Felder | Beschreibung |
  | :----- | :----------- |
  | Bedienerklasse | Eindeutige numerische Identifikation der Bedienerklasse |
  | Bezeichnung | Bezeichnung der Bedienerklasse |
  | Betriebsstätte | <p>Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.</p><p>Standard: 0, ohne Filiale</p><p>Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden.</p> |
  | Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte |
  | Login-Sperre | <p>JA/NEIN</p><p>Login-Sperre aller Bediener dieser Bedienerklasse</p> |
  | Formular FiBu-Infofenster | <p>JA/NEIN</p><p>Zusätzliche Informationen in der Finanzbuchhaltung aktivieren</p> |
  | Toolbar aktiv | <p>JA/NEIN</p><p>Aktiviert die standardmäßig ausgelieferte Toolbar.</p> |
  | Abteilung | <p>Hier kann die Bedienerklasse einer Abteilung zugeordnet werden.</p><p>Abteilungen</p> |
  | Controllerklasse | <p>Controllerklasse</p><p>Controllerklassen werden gelb hervorgehoben.</p> |
  | Sicherheitsklasse | Sicherheitsklasse |
</details>

<details>
  <summary>Formulararchiv</summary>

  Hier kann für eine Bedienerklasse festgelegt werden, welche Formulare des Archivs (abhängig von den zugeordneten Bedienerklassen) eingesehen werden dürfen.

  Diese Rechte können auch beim Bediener (Register Formulararchiv) eingesehen werden.
</details>

<details>
  <summary>Passwortrichtlinien</summary>

  | Felder | Beschreibung |
  | :----- | :----------- |
  | Mindestlänge | Wie viele Zeichen das Passwort mindestens haben soll. |
  | Höchstlänge | <p>Wie viele Zeichen das Passwort maximal haben darf.</p><p>Das Limit liegt bei 10 Zeichen.</p>
  | Zahlen | Wie viele Zeichen im Passwort mindestens eine Zahl (0-9) sein müssen. |
  | Sonderzeichen | <p>Wie viele Zeichen im Passwort mindestens ein Sonderzeichen sein müssen.</p><p>Ausgenommen sind folgende Zeichen: „ \ ; - ‘</p> |
  | Aktualisierung in Tagen | Nach wie vielen Tagen seit der letzten Passwortänderung das Passwort wieder geändert werden muss. |

  **Die Passwortrichtlinien werden je nach Bedienerklasse individuell verwaltet und gepflegt.**

  **Die Bearbeitung und Anpassung der Passwortrichtlinien ist ausschließlich Systemadministratoren vorbehalten.**

  Zur Änderung des eigenen Passworts kann über den Direktsprung **[PWD]** die entsprechende Ansicht zur Passwortänderung geöffnet werden.

  Wird ein Wert auf **0** gesetzt oder das Kontrollkästchen deaktiviert, so wird die jeweilige Bedingung bei der Prüfung nicht berücksichtigt.

  Die Richtlinie **„Aktualisierung in Tagen“** wird nur beim Anmeldevorgang in **A.eins** überprüft.
</details>

<details>
  <summary>Bedienerklasse: Pfleger Funktionen</summary>

  Im Pfleger stehen folgende Funktionen zur Verfügung:
  
  |    | Funktionen |
  | :--- | :--------- |
  | Speichern **F9** | Speichert die Daten |
  | Speichern unter … **Shift+F9** | <p>Übernimmt die Daten der ausgewählten Bedienerklasse.</p><p>Das System schlägt eine neue Bedienerklasse vor.</p><p>Die Einstellungen hinsichtlich des Formulararchivs sind in diesem Modus nicht zu beeinflussen, die Registerkarte „Formulararchiv“ wird ausgeblendet.</p><p>Bei Ausführung der Funktion **Speichern F9** wird die neue Bedienerklasse zur weiteren Verwendung im System vorbereitet (der Vorgang kann etwas dauern und sollte nicht abgebrochen werden):</p><p>**Es werden dabei u.a. sämtliche Schutzeinstellungen der Funktionen** (Zugriffsrechte Funktionen) **der Ursprungsbedienerklasse übernommen**.</p><p>Ferner werden dabei alle Einstellungen der Ursprungsbedienerklasse „geclont“.</p><p>Bei den Formulararchiv-Einstellungen sind die Belege der Ursprungsklasse erlaubt, in den Einstellungen der Ursprungsklasse sind die Einstellungen der neuen Bedienerklasse erlaubt.</p> |
  | Löschen **F7** | <p>Bedingung: Die Bedienerklasse darf keine Bediener mehr enthalten.</p><p>Alle Einstellungen der Bedienerklasse in den abhängigen Einstellungen (Zugriffsrechte Funktionen, EPA, …) werden herausgenommen.</p><p>Zum jetzigen Zeitpunkt betrifft dies folgende Tabellen:</p><ul><li>NUMKREISBEDIENER</li><li>KORRKLASSBEDLINK</li><li>BEDIENERPROFIL</li><li>ABTEILBEDIENLINK</li><li>OWAAGENUMKREIS</li><li>FIBUBELEGNUMKR</li><li>BEDIENERGRUPLINK</li><li>USERFELDER</li><li>KONTRKLNUMKREIS</li><li>FORMULARARCHIVBEDIENER</li><li>AMIC_NEWS</li><li>FA_IMPORT</li><li>FA_IMPORT_REGEL</li><li>FORMULARARCHIV_MANAGER</li><li>AMIC_MENU_MESSAGES</li><li>FA_VIEW_PROFIL</li><li>FA_VIEW_PROFIL_DETAIL</li><li>ANWENDFUNKKLASSE</li><li>ANWENDOBKLASSE</li><li>ANWENDVARKLASSE</li><li>AMICINFOSYSMSKZUORD</li><li>ROLLENKLASSE</li></ul> |
  | Neu **F8** | <p>Neue Bedienerklasse anlegen.</p><p>Siehe aber auch "Speichern unter ... SF9" für die Anlage einer neuen Bedienerklasse mit gleichzeitiger Übernahme der Bedienerklassen-relevanten Einstellungen!</p> |
  |EPAs zeigen **F10** | <p>Individuelle Steuerungen von Abläufen können in Anwendungen über Einrichterparameter (EPA) vorgenommen werden.</p><p>Diese Funktion ruft die entsprechende Anwendung zur Ansicht und Pflege der Einrichterparameter auf.</p><p>Hauptmenü > Administration > Steuerung > EPAs zeigen</p><p>Direktsprung **[EPAZ]**</p> |
  | Zugriffsrechte übertragen… | |
  | NKZ-Zuordnung | Nummernkreiszuordnung der ausgewählten Bedienerklasse |
  | Betriebsstätte | <p>Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.</p><p>Standard: 0, ohne Filiale</p><p>Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden.</p> |
  | Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte |
</details>

