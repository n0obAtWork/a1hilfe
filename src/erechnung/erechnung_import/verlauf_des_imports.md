# Verlauf des Imports

<!-- source: https://amic.de/hilfe/verlaufdesimports.htm -->

Nachdem eine eRechnungs-XML im Formulararchiv abgelegt wird, kann diese importiert werden.

Die Schritte, die hier gemacht werden, sind:

1. Prüfung, ob es sich bei dem markierten Eintrag um einen der Belegklasse **8045** (eRechnung-Import-Xml).handelt

2. Prüfung, ob es sich beim Xml um eines der Formate UBL-Invoice, UBL-CreditNote oder CII handelt.

3. Validierung des Xml nach den Regeln der KoSIT

4. Einlesen der Daten in die Tabellen XRe

5. Ggf. die Ermittlung eines Kunden/Lieferanten

Bei der Verwendung von [FAI] und [EMAIL] werden diese Schritte automatisch angestoßen.

Beim manuellen Import muss dies mit der Funktion eRechnung bearbeiten manuell für den Eintrag aufgerufen werden.
