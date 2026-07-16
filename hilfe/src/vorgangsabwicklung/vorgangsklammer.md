# Vorgangsklammer

<!-- source: https://amic.de/hilfe/_vorgangsklammer.htm -->

Die Vorgangsklammer gilt als Stammsatz für die Vorgangsmappe.

Das Löschverhalten einer Vorgangsklammer kann über den Steuerparameter „Löschverhalten bei Vorgangsmappen“ ([SPA 796](../firmenstamm/steuerparameter/optionen_warenwirtschaft/loeschverhalten_bei_vorgangsmappen_spa_796.md)) beeinflusst werden.

#### Kopffelder

| Feld | Beschreibung |
| --- | --- |
| Klammernummer | Nummer der Klammer |
| Bezeichnung | Bezeichnung der Klammer |
| Container | Hier kann eine Containernummer hinterlegt werden.<br>Zusätzlich kann hier auch eine Itembox zur Verfügung stehen. Diese wird im [Streckenerfassungsprofil](../zusatzprogramme/streckenerfassung/profile/index.md#ProfileStreckenerfassung) unter „Itembox für Container“ eingetragen. Bei mehreren Profilen wird die erste gefundene Itembox verwendet.<br>Wenn in der Itembox ein Rückgabefeld mit dem Namen „retBezeichnung“ zurückgegeben wird, so erscheint dieser Wert in dem Anzeigefeld hinter dem Containerfeld. |
| Abw. Adresse | Hier kann eine Abweichende Adresse angegeben werden.<br>Siehe [unten](./vorgangsklammer.md#Adresszuordnung) für weitere Adressenpflege und Zuordnung. |

#### Registerkarte „Vorgänge“

| Feld | Beschreibung |
| --- | --- |
| Führende Elemente aus Grid 1 | Frei eingebbarer Text |
| Mengenabhängige Elemente aus Grid 2 | Frei eingebbarer Text |
| Weitere abhängige Elemente aus Grid 3 | Frei eingebbarer Text |
| Packer | Bediener dieser Klammer |
| Status | Status der Klammer, zulässige Werte können im Anwenderformat „AF_KLSTATUS“ gepflegt werden. |
| Lokalität | Lokalität aus dem Lagerverwaltungssystem |
| Abweichende Zieladresse | Über den zugehörigen Button mit dem Fragezeichen kann man eine alternative Adresse aus dem Anschriftstamm auswählen.<br>Wählt man den Button mit dem Stift kann man die abweichende Zieladresse neu eingeben oder ändern. |
| Abweichende Herkunftsadresse | Über den zugehörigen Button mit dem Fragezeichen kann man eine alternative Adresse aus dem Anschriftstamm auswählen.<br>Wählt man den Button mit dem Stift kann man die abweichende Herkunftsadresse neu eingeben oder ändern. |
| Spedition | Über den zugehörigen Button mit dem Fragezeichen kann man eine Speditionsadresse aus dem Anschriftstamm auswählen.<br>Wählt man den Button mit dem Stift kann man die Speditionsadresse neu eingeben oder ändern. |
| Kontrolleur | Über den zugehörigen Button mit dem Fragezeichen kann man eine Kontrolleuradresse aus dem Anschriftstamm auswählen.<br>Wählt man den Button mit dem Stift kann man die Kontrolleuradresse neu eingeben oder ändern. |
| Löschkontrolleur | Über den zugehörigen Button mit dem Fragezeichen kann man eine Löschkontrolleuradresse aus dem Anschriftstamm auswählen.<br>Wählt man den Button mit dem Stift kann man die Löschkontrolleuradresse neu eingeben oder ändern. |

#### Registerkarte „Tour“

| Feld | Beschreibung |
| --- | --- |
| Tour | Frei eingebbarer Text |
| Lieferdatum | Frei eingebbares Datum |
| Archivreferenz | Referenznummer fürs Archiv, wird bei der Neuanlage automatisch vergeben. |
| Element 1 | Frei eingebbarer Text |
| Element 2 | Frei eingebbarer Text |
| Element 3 | Frei eingebbarer Text |

#### Registerkarte „Ladetermine“

Auf dieser Registerkarte steht eine Liste zur Verfügung, in der man Termine eintragen kann.

| Feld | Beschreibung |
| --- | --- |
| Termin | Datum des Termins |
| Ladeschein | Frei eingebbarer Text für eine Ladescheinnummer |
| Status | Status des Termins, zulässige Werte können im Anwenderformat „AF_KLTERMIN“ gepflegt werden. |
| Menge | Frei eingebbare Menge |
| Ladenummer | Frei eingebbarer Text für eine Ladenummer |
| Termin / Uhrzeit | Frei eingebbarer Text für einen Termin / Uhrzeit |

#### Registerkarte „Drucktexte“

| Feld | Beschreibung |
| --- | --- |
| Termin | |
| Parität | |
| Erfüllung | |
| Schiff | |
| Befrachter | |
| Kontrolle | |
| Abruf | |
| Text 11 | |
| Text 12 | |
| B/L Datum | |
| Kontraktzeitraum | |
| Kontraktparität | |
| Frachtzahler | |
| Anbauland | |
| Bitte … | Bitte arrangieren Sie alles für eine ordnungsgemäße Entladung. |
| Hiermit…. | Hiermit garantieren wir, dass |
| Unbedingt …. | Bitte unbedingt den beigefügten Lieferschein bei jeder Lieferung verwenden! |
| Menge | |
| Menge Kontrolle | ca. / min. / max. |
| Kontrollsatz | € / mt ; € / Stunde; € / 8Stunden |

#### Registerkarte „Bemerkungen“

| Feld | Beschreibung |
| --- | --- |
| Anliefer Avis | |
| Ansprechpartner Anliefer Avis | |
| Andienung | |
| Ansprechpartner Andienung | |
| Transportauftrag | |
| Ansprechpartner Transportauftrag | |
| Nominierung | |
| Ansprechpartner Nominierung | |
| Freistellung | |
| Ansprechpartner Freistellung | |
| Abruf | |
| Ansprechpartner Abruf | |
| Verladeinfo | |
| Ansprechpartner Verladeinfo | |
| Kontrolle | |
| Ansprechpartner Kontrolle | |

#### Adresszuordnungen

Zusätzlich zu der abweichenden Adresse innerhalb der Streckenverarbeitung können auf der ersten Streckenseite noch eine Adresse des zugehörigen Spediteurs und eine Adresse des zugehörigen Kontrolleurs zugeordnet und gepflegt werden.

Diese Adressen werden bei der Dokumentverarbeitung zur Auffüllung des Adressfeldes genutzt.

Es stehen hier einerseits Funktionen zu jeweiligen Auswahl einer Adresse zur Verfügung (…), wie auch Funktionen zu Bearbeitung einer Adresse des bestimmten Bereiches.

Bei der Bearbeitung gibt es wiederum die Möglichkeit der Änderung oder der komplette Neueingabe.
