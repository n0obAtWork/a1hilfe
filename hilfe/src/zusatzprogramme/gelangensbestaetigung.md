# Gelangensbestätigung

<!-- source: https://amic.de/hilfe/_gelangensbestaetigung.htm -->

Warenverkauf > Übergreifend > Gelangensbestätigung

### Gesetzeslage

Die steuerrechtliche Lage seit 2012: Wird Ware an einen Kunden im Ausland verkauft, so geschieht dies steuerfrei. Wird dem Finanzamt jedoch bei einer Prüfung nicht nachgewiesen, dass die Ware auch tatsächlich ins Ausland geliefert wurde, so kann eine Nachbesteuerung angeordnet werden.

Zu diesem Zwecke ist mit den Lieferpapieren eine Gelangensbestätigung auszuliefern, die vom Empfänger quittiert und zurückgesandt wird. Diese ist dann zu archivieren.

Seitens des Finanzamtes könnte im Fall einer Prüfung die Nachbesteuerung unbelegter Lieferungen angeordnet werden. In diesem Fall wenden Sie sich an die Steuerberater bzw. die zuständigen Finanzbehörden zur Klärung des Vorgehens.

### Einrichtung der Gelangensbestätigung

#### SPA

Es ist der [Lizenz-SPA 865 – Gelangensbestätigung](../firmenstamm/steuerparameter/lizenzen/gelangensbestaetigung_lizenz_spa_865.md) notwendig.

Die Gelangensbestätigung wird in der Regel mit dem Lieferschein im Vorgangsdruck mittels eines eigenen Formulars erstellt. Jene Steuergruppen für die eine Steuerfreiheit gilt, weil die Lieferung ins Ausland erfolgt, können im Steuerparameter [SPA 830](../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/steuergruppen_gelangensbestaetigung_spa_830.md) eingetragen werden.

Wird ein Lieferschein mit dieser Steuergruppe erstellt, so wird für diesen Vorgang beim Formulardruck das Erstellungsdatum der Gelangensbestätigung festgehalten.

[SPA 948 – Gelangensbestätigung bei Belegkorrektur](../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/gelangensbestaetigung_bei_belegkorrektur_spa_948.md).

Der Steuerparameter SPA 948 entscheidet, ob nach Belegkorrektur eine neue Gelangensbestätigung gedruckt wird. Der Standardwert ist „Nein“. Wenn nach der Korrektur eines Beleges eine neue Gelangensbestätigung gedruckt werden muss, ist im Steuerparameter SPA 948 der Wert „Ja“ einzutragen.

#### Formular und Makro

Der Anschluss des Drucks wird über die [Vorgangsdruckklassen](../firmenstamm/druckereinrichtung/vorgangsdruckklassen.md) (VRGD) geregelt. Wir liefern ein **Muster-Formular (-100)** mit aus, das nötigenfalls sogar ohne Kopie und Anpassung derselben direkt zu verwenden ist. Als Rücksendeadresse wird hier die Mandantenanschrift benutzt.

Ferner fungiert das Mustermakro **AMIC_GelangensBest_Muster** als VRGD Entscheider-Makro. Dieses Makro liefert die Entscheidung, ob zu drucken ist oder nicht und markiert nach erfolgreichem Druck diesen Beleg als gelangensrelevant. Überdies ist das Makro in seiner jetzigen Ausprägung gleichzeitig über JVAR Steuerungen für die mehrsprachige Aufbereitung des Musterformulars -100 (Deutsch, Englisch, Französisch) vorbereitet.

Da das Mustermakro keinen erneuten Druck erlaubt, kann dies mit Hilfe des [Lieblingsdruckerdrucks](../vorgangsabwicklung/vorgangsbearbeitung/lieblingsdruckerdruck.md) erfolgen. Dort existiert ein Haken, dass die Makros beim VRGD Druck ausgeführt werden sollen. Das Mustermakro ist so ausgelegt, dass es beim Lieblingsdruckerdruck ein erneutes Drucken zulässt.

Bei individuellem Makro kann das Kennzeichen für den Lieblingsdruckerdruck mithilfe der JVARS abgerufen werden.

```text
jvarsget(3567 , "IstLieblingsdrucker", vIstLieblingsdruckerBuffer, 256);
```

### Bearbeitung der Gelangensbestätigung

Warenverkauf > Übergreifend > Gelangensbestätigung

In der Anwendung Gelangensbestätigung **[GELB]** können alle Vorgänge angezeigt werden, für die eine Gelangensbestätigung ausgestellt wurde.

Ihnen stehen nach Auswahl und Markierung für die Bearbeitung folgende Funktionen zur Verfügung:

#### Mahnung eintragen

Hier können Sie zu diesem Beleg das aktuelle Datum als Datum einer Erinnerung für die Rücksendung der Gelangensbestätigung setzen. Die Erstellung/Durchführung bzw. das Versenden der Erinnerung ist manuell durchzuführen.

#### Rückmeldung eintragen

Hier können Sie das aktuelle Datum als das Datum des Eingangs/der Bearbeitung der Rückmeldung eintragen.

#### Löschen

Hier können Sie die Führung der Gelangensbestätigung zum Beleg löschen.

### Mahnschreibendruck

Um den Mahnschreibendruck durchführen zu können muss in den [Vorgangsunterklassen](../vorgangsabwicklung/formularzuordnung/formular_formularzuordnungen_zum_vorgang_unterklasse.md) ein Gelangensmahnformular hinterlegt werden. Danach ist es möglich für mehrere Belege hintereinander den Mahnschreibendruck durchzuführen.
