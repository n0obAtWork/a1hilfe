# Forderungsgruppen

<!-- source: https://amic.de/hilfe/forderungsgruppen.htm -->

Hauptmenü > Stammdatenpflege \> Konstanten Kundenstamm > Forderungsgruppe

Direktsprung **[FORG]**

Die aus den einzelnen Rechnungen / Offenen Posten eines Personenkontos resultierenden Forderungen bzw. Verbindlichkeiten müssen auf den Bestandskonten verbucht und bei Bezahlung wieder ausgebucht werden. In A.eins ist es möglich die Kunden in verschiedene Gruppen einzuteilen z.B. **"Großkunden", "Landwirte", "Baustoffhändler"** und diesen Gruppen auch unterschiedliche Bestandskonten zuzuordnen. Hierzu dienen die Forderungsgruppen.

| Feld | Beschreibung |
| --- | --- |
| Nummer<br> | Identifikation der Forderungsgruppe. Diese wird als Verweis im Kunden-/Lieferantenstamm hinterlegt.<br> |
| Bezeichnung<br> | Die Bezeichnung dient lediglich zur textlichen Beschreibung der Forderungsgruppe und hat keine Programmfunktion. Sie wird in Auswahllisten bzw. Reporten zur Anzeige verwendet.<br>Ist der Steuerungsparameter 34 "Mehrsprachigkeit aktiv“ in A.eins gesetzt, so hat man auf diesem Feld die Möglichkeit mit F3 [sprachabhängige Bezeichnungen](../../firmenstamm/a_eins_sprache/sprachabhaengige_bezeichnung_in_den_stammdaten.md) zu pflegen.<br> |
| Konto Forderungen/ Konto Verbindlichkeiten<br> | In A.eins können Personenkonten sowohl Forderungen als auch Verbindlichkeiten ausweisen (Kontokorrent- Kunden). Daher ist es notwendig in der Forderungsgruppe sowohl ein Forderungs- als auch Verbindlichkeitskonto anzugeben. Die Konten können mit **F3** ausgewählt werden. In dieser F3-Auswahl werden nur Bilanzkonten angeboten, deren Erfassungssperre gesetzt ist, die kein Steuerkonto sind und die nicht als Erlöskonto verwendet werden.<br>**ACHTUNG: Forderungs- und Verbindlichkeitskonten dürfen nicht direkt bebucht werden.**<br>Wie die Forderungs- und Verbindlichkeitskonten eingerichtet werden sollten, hängt zum einen von der Firmeneigenen Struktur und zum anderen von dem Steuerungsparameter „Methode der Forderungs-Verb-Zuordnung“ ab.<br> |
| Ersatzkonto DATEV | Dieses Feld erscheint nur, wenn das DATEV-Modul aktiv ist und man im DATEV-Firmenstamm angegeben hat, dass man ein Ersatzkonto aus der Forderungsgruppe verwenden will. Dieses Ersatzkonto wurde aus zwei Gründen eingeführt.<br>1. Die Personenkonten wurden eingerichtet, bevor man sich im Unternehmen entschieden hatte, die DATEV-Schnittstelle zu verwenden und die Nummern der Personenkonten nicht DATEV-konform eingerichtet worden sind.<br>2. Man hat sich entschieden, dem Steuerberater keine genauen Informationen über die Personenkonten mitzugeben.<br> |
| Gesperrt | Forderungsgruppe sperren. Gesperrte Forderungsgruppen werden in der F3-Auswahl von der Anzeige ausgeschlossen. So können gesperrte Forderungsgruppen nicht im Kundenstamm ausgewählt werden.<br> <br>Vorbelegung: Nein.<br> |

Wird in einer Forderungsgruppe das Forderungs- oder Verbindlichkeitskonto geändert und existieren in **nicht abgeschlossenen** Perioden Belege, in denen Personenkonten mit dieser Gruppe vorkommen, so wird ein Kennzeichen gesetzt, dass eine [Reorganisation](../fibu_reorganisator/reorganisation_starten.md) notwendig ist. Das Ändern von Forderungsgruppen, für die bereits in **abgeschlossenen** Perioden Belege existieren, ist nicht mehr möglich.

Auch das Ändern der Forderungsgruppe im [Kundenstamm](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/fibu_merkmale.md#Verbuchungsmerkmale) führt dazu, dass dieses Kennzeichen gesetzt wird. Vor dem Druck von Auswertungen und vor dem Abschluss von Perioden wird dieses Kennzeichen geprüft und ggf. eine entsprechende Meldung ausgegeben.

#### Methode der Forderungs- und Verbindlichkeitszuordnung

In A.eins existieren zwei Möglichkeiten, wie aus Rechnungen die Zuordnung zu den Bestandskonten durchgeführt wird.

**Standard**  
Bei der Methode Standard ist es so, dass eine Ausgangsrechnung die Forderungen erhöht, eine Ausgangsgutschrift auf diese Ausgangsrechnung diese wieder vermindert. Eingangsrechnungen erhöhen die Verbindlichkeiten und Gutschriften auf diese vermindern die Verbindlichkeiten. Zahlungen, die nicht sofort einer Rechnung zugeordnet ( ausgeziffert ) werden, werden je nach Sollhabenkennzeichen als Verminderung der Forderungen bzw. als Verminderung der Verbindlichkeiten interpretiert. Alle anderen Belegarten ergeben sich aus ihrem Bezug zu den Rechnungen. Das bedeutet aber auch, dass es vorkommen kann, dass einem Lieferanten plötzlich Forderungen zugeordnet werden, wenn er aus irgendeinem Grunde eine Ausgangsrechnung/Gutschrift zugeordnet bekommt. Dies kann man umgehen, indem man für Kunden, Lieferanten und Kontokorrentkunden unterschiedliche Forderungsgruppen einrichtet. Diese Einrichtung könnte dann folgendermaßen aussehen:  
    

| | Forderungskonto | Verbindlichkeitskonto |
| --- | --- | --- |
| Kunden | 1400 | 1400 |
| Lieferanten | 1600 | 1600 |
| Kontokorrent | 1401 | 1601 |

Dadurch werden die Beträge, die bei Kunden den Verbindlichkeiten zugeordnet würden, trotzdem auf das Bestandskonto für Forderungen gehen. Bei Lieferanten entsprechend die Forderungen auf das Bestandskonto für Verbindlichkeiten. Bei den Kontokorrentkunden, bei denen sowohl Forderungen als auch Verbindlichkeiten auftreten können, braucht man natürlich die Trennung der Konten.  
    
**Hinweis**: Wenn man die Methode <strong>„Standard“</strong> verwendet und Eingangs- und Ausgangsrechnungen in der OP-Verwaltung miteinander verrechnet bzw. Belege mit unterschiedlicher Forderungs- und Verbindlichkeitszuordnung miteinander verrechnet, kommt es, wenn Forderungs- und Verbindlichkeitskonto unterschiedlich sind, zu einer Besonderheit:

| | Betrag | Forderung | Verbindlichkeit |
| --- | --- | --- | --- |
| Eingangsrechnung | 2.000,00 H | 0,00 S | 2.000,00 H |
| Verrechnete Ausgangsrechnung | 500,00 S | 500,00 S | 0,00 H |
| Zahlung | 1.500,00 S | 0,00 S | 1.500,00 S |
| | | | |
| Summe: | 0,00 S | 500,00 S | 500,00 H |

Es würden also, obwohl der Lieferant/Kontokorrentkunde einen ausgeglichenen Saldo hat, jeweils 500,00 S Forderungen und 500,00 H Verbindlichkeiten offenbleiben. Der Saldo aus Forderungen und Verbindlichkeiten ist zwar 0, jedoch werden diese in Auswertungen unterschiedlich bewertet, so dass dies nicht richtig wäre. Daher kommt es zu sogenannten **Internen Umbuchungen**. Dies sieht dann wie folgt aus:  
    

| | Betrag | Forderung | Verbindlichkeit |
| --- | --- | --- | --- |
| Eingangsrechnung | 2.000,00 H | 0,00 S | 2.000,00 H |
| Verrechnete Ausgangsrechnung | 500,00 S | 500,00 S | 0,00 H |
| Zahlung | 1.500,00 S | 0,00 S | 1.500,00 S |
| Interne Umbuchung | 500,00 H | 500,00 H | 0,00 H |
| Interne Umbuchung | \-500,00 H | 0,00 H | 500,00 S |
| | | | |
| Summe: | 0,00 S | 0,00 S | 0,00 H |

Diese Interne Umbuchung, die im Saldo für den Kunden neutral bleibt, sorgt dafür, dass die Forderungs- und Verbindlichkeitskonten ausgeglichen werden. Interne Umbuchungen sind **NIE** offene Posten. Wird eine Auszifferung zurückgesetzt, werden diese Internen Umbuchungen gelöscht. Sind diese bereits gebucht, so werden automatisch Stornobelege erstellt und gegen die Interne Umbuchung ausgeziffert.

**Saldo Stichtag**  
Bei der Methode „Saldo Stichtag“ bestimmt der Saldo des Kunden in der aktuellen Periode, welcher Seite der Betrag zugeordnet wird. Hat der Kunde einen Sollsaldo, dann wird der Betrag dem Forderungskonto zugeordnet, hat er einen Habensaldo, dann dem Verbindlichkeitskonto. Diese Methode bringt ein paar Einschränkungen mit sich:  
    
a) Die Forderungsgruppen müssen so eingerichtet sein, dass Forderungs- und Verbindlichkeitskonto unterschiedlich sind.  
    
b) Kontoblätter für die Forderungs- und Verbindlichkeitskonten können erst gedruckt werden, wenn die Perioden abgeschlossen sind.  
    
c) Eine Verprobung der Salden der Kunden mit den Bestandskonten lässt sich nur dadurch erreichen, dass man bei den Saldenlisten Debitoren/Kreditoren die Abfrage „Konten im S/H“ einmal im Soll für dir Abstimmung mit den Forderungskonten und einmal im Haben für die Abstimmung mit den Verbindlichkeitskonten druckt.
