# Standard Bedienerklassen

In A.eins gibt es mehre Standardklassen, welche genutzt werden können.

<details>
  <summary>Defaultklasse</summary>

  Diese Bedienerklassen werden von A.eins zur Abwicklung weiterführender Programmmodule benötigt und sollten nicht geändert werden.
  
  Momentan sind folgende Bedienerklassen technisch notwendig:
  
  |    | Technische Bedienerklassen |   
  | :--- | :------------------------- |
  | -1 | Defaultklasse Kunden |
  | -9999 | Defaultklasse AMIC |
</details>

<details>
  <summary>Controllerklasse</summary>

  Bedienerklassen die Mitglieder der Controller-Rolle sind Controllerklassen.
</details>

<details>
  <summary>Sicherheitsklasse</summary>

  Es ist möglich, A.eins mit der neu eingeführten „Sicherheitsklasse“ in einen Schutzsystem-Zustand zu überführen: In diesem haben nur noch die Mitglieder der „Sicherheitsklasse“ – sogenannte „Technische Administratoren“ Zugriff auf die Änderung der Schutzeinstellungen der Anwendungsfunktionen in A.eins (siehe auch Zugriffsrechte Funktionen).

  Praktisch bedeutet dies, dass den „Technischen Administratoren“ nur die Anwendung „Zugriffsrechte Funktionen“ (Zugriffsrechte Funktionen) und Teile der Anwendung „Bedienerstamm“ (Einrichtung Bediener]) zur Verfügung stehen – keine weiteren.

  Außer den Mitgliedern der Sicherheitsklasse werden „Schutzänderungen“ in „Dieses Menü“ nicht mehr erlaubt. Stattdessen wird den Mitgliedern ein sogenannter „Schutzantrag“ per Mail über den Datenbank-Server zugestellt. Das A.eins-System verifiziert die gewünschte letzte Änderung auf Gültigkeit. Der „Technische Administrator“ darf keinen Schutz so verändern das er nach der Änderung selber mehr und auch nicht weniger Rechte hat.

  Im Bedienerstamm sind nach Aktivierung des „Sicherheitsklassen“-Status die Funktionen des Bedienerklassenwechsels und Änderungen der „Name Sicherheit“-Einstellungen sowie diverse Situationen beim Ändern und Speichern derart geändert worden, dass sie die Situation eines aktiven „Sicherheitsstatus“ gewährleisten und unterstützen.

  > [!CAUTION]
  > **Das Aktivieren der Einstellung „Sicherheitsklasse“ ist nicht rückgängig zu machen.**
</details>