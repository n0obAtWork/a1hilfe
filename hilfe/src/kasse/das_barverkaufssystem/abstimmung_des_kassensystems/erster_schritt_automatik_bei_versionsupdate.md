# Erster Schritt: Automatik bei Versionsupdate

<!-- source: https://amic.de/hilfe/ersterschrittautomatikbeiversi.htm -->

Um eine Abstimmung zwischen Kasse und Fibu technisch überhaupt möglich zu machen, waren weitere Vorkehrungen notwendig. Ab der Version 6.3 wird eine definierte Verbindungstabelle namens AcashFibuLink geführt, die die Beziehungen zwischen Kassenbelegen und ihren zugehörigen Fibu-Belegen definiert. Da diese Tabelle neu eingeführt wurde, müssen die Beziehungen bereits bestehender Objekte erstmalig hergestellt werden. Das leistet ein Umstellprogramm, das in den Update Prozess eingebettet ist. Dieses Programm versucht, aufgrund im Kassensystem vorhandener Daten (Zahlungsbetrag, Datum, Belegart, Kundennummer, Kassenkonto, Gegenkonto, Buchungstexte usw.) einen passenden Fibu-Beleg zu finden. Nur wenn ein eindeutig passender Beleg gefunden wird, kann dieser automatisch zugewiesen werden.
