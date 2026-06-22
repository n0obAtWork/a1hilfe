# Teildisposition mit Vorlauf

<!-- source: https://amic.de/hilfe/_teildisposition_mit_vorlauf.htm -->

Eine besondere Art der Teildisposition ist die „Teildisposition mit Vorlauf“, die über den [Steuerparameter 986 („Teildisposition mit Vorlauf“ aktiv)](../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/teildisposition_mit_vorlauf_aktiv_spa_986.md) aktiviert werden kann. Sie kann an den folgenden Stellen aufgerufen werden:

• Angebotsbearbeitung [AGB], Funktion Teildisposition Auftrag

• Angebotsbearbeitung [AGB], Funktion Teildisposition Lieferschein

• Auftragsbearbeitung [AUB], Funktion Teildisposition Lieferschein

• Auftragsbearbeitung [BSB], Funktion Teildispo in Eingangslieferschein

• Auftragsbearbeitung [BSB], Funktion Teildispo in Eingangsrechnung

• Auftragsbearbeitung [BSB], Funktion Teildispo in Ladeschein

• Auftragsbearbeitung [LIB], Funktion Teildisposition Rechnung

Bei der Teildisposition mit Vorlauf wird ein neuer Ziel-Vorgang angelegt, bei dem die Daten aus dem Quell-Vorgang übernommen werden, die auch im neuen Vorgang Sinn ergeben. Andere Daten wie erfassender Bediener und Erfassungsdatum werden dagegen neu gesetzt. Ist die Lagernummerfehl ein Sortimentslager, so wird die Lagernummerfehl auf die Lagernummer aus den Vorgangskonstanten gesetzt.
