# Artikelmaske

<!-- source: https://amic.de/hilfe/_ktrartikelmaske.htm -->

Auf dieser Maske lassen sich die Artikel für den Kontrakt bearbeiten und einfügen. Ein Artikel kann dabei auch mehrfach in einem Kontrakt geführt werden, z.B., um unterschiedliche Qualitäten eines Artikels mit verschiedenen Preisen versehen zu können. Beim Fakturieren werden dann alle Alternativen, diesen Artikel und Kunden betreffend, angezeigt.

Des Weiteren können auf dieser Maske individuelle Felder über das „A.eins Informationssystem (AIS)“ zugeordnet werden. Beim Einrichten des AIS müssen jedoch einige Besonderheiten beachtet werden. (siehe dazu „[Kontraktartikel (AIS)](../../../zusatzprogramme/ais_a_eins_informationssystem/private_tabellen/kontraktartikel_ais.md)“)

Für die Artikelmaske stehen folgende Felder zur Verfügung, nach dem Speichern der Daten wird die Maske verlassen und zurück zur Anzeigemaske gesprungen.

| Feld | Beschreibung |
| --- | --- |
| Laufende Nummer im Kontrakt | Vorgeschlagen wird die nächste freie Nummer entsprechend der Erfassungsreihenfolge. Diese Nummer steuert die Reihenfolge der Warenpositionen im Ausdruck von Listen. |
| Lagernummer | Hier kann die Lagernummer des Artikels eingegeben werden. Dieses Feld ist nur aktiv, wenn der Kontrakt lagerspezifisch ist oder das Lagerspezifisch Feld aktiviert wurde. |
| Artikelnummer | Artikelnummer der Artikelposition |
| Lagerspezifisch | Diese Feld ist nur beim ersten Artikel aktiv und wenn der Kontrakt nicht lagerspezifisch ist. Wird das Feld aktiviert, erscheint das Lagernummernfeld und die Itembox für lagerspezifische Artikel wird auf dem Artikelnummernfeld angezeigt.  
Über einen [Einrichterparameter](../../../firmenstamm/einrichterparameter/kontraktartikel_epa_ktrartin.md) lässt sich das Feld in den Kontraktstamm übernehmen. |
| Rohwarengruppe  
*(nur Rohwarekontrakte)* | Dieses Feld wird bei der Auswahl des Artikels vorbelegt und sollte nur als Anzeigefeld dienen. Es kann jedoch über einen [Einrichterparameter](../../../firmenstamm/einrichterparameter/kontraktartikel_epa_ktrartin.md) zum Bearbeiten freigeschaltet werden. |
| Rohwarensorte  
*(nur Rohwarekontrakte)* | Hier kann die Rohwarensorte des Artikels angegeben werden, die Sorte wird durch den Artikel vorbelegt, kann jedoch auch über eine private Itembox auf dem Artikel vorbelegt werden.  
Dafür muss in der privaten Itembox ein zusätzliches Feld geliefert werden. („RohSorteNummer“) |
| Kontraktmenge | Vereinbarte Menge der Artikelposition. |
| Kontraktpreis | Vereinbarter Preis der Artikelposition. |
| Je | Dieses Feld bezieht sich mit auf den Preis und die Mengeneinheit. |
| Mengeneinheit | Die Mengeneinheit die sich mit auf das „Je“ und „Preis“ Feld bezieht. |
| Kurs | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet.  
Stellt den Währungskurs zum Kontraktartikel als Faktor zur Buchwährung dar. |
| Frachtzu-/abschlag | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Frachtsatz | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Tradebasis | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Handling | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Pricing | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Future | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Hedge – Artikel | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Hedge – Monat | Informatives Feld zur Bewertung. Wird aktuell nur in (privaten) Reports oder Auswahllisten verwendet. |
| Allg. Wert | Hier kann ein allgemeiner Wert für diesen Kontraktartikel hinterlegt werden. Über einen [Einrichterparameter](../../../firmenstamm/einrichterparameter/kontraktartikel_epa_ktrartin.md) kann eine Prozedur zur Berechnung hinterlegt werden, dann lässt sich das nicht mehr von Hand bearbeiten. |
| Bemerkung | Bemerkung die für diese Artikelposition hinterlegt werden kann. |
| Absch  
*(nur Rohwarekontrakte)* | Abschlagspreis für den Rohwareartikel |
| Folge  
*(nur Rohwarekontrakte)* | Folgeabschlag für den Rohwareartikel |
| Mind  
*(nur Rohwarekontrakte)* | Mindestabschlag für den Rohwareartikel |
| WM-Preis  
*(nur Rohwarekontrakte)* | Weltmarktpreis für den Rohwareartikel. Über einen [Einrichterparameter](../../../firmenstamm/einrichterparameter/kontraktartikel_epa_ktrartin.md) kann dieser automatisch aus dem Preis übernommen werden. |
| Rohwareabschlagssatz  
*(nur Rohwarekontrakte)* | |
| Planlieferdatum | Dieses Feld dient zum Speichern von weiteren Informationen. Über einen [Einrichterparameter](../../../firmenstamm/einrichterparameter/kontraktartikel_epa_ktrartin.md) kann das Feld weggeschaltet werden. |
| Planlieferzeit | Dieses Feld dient zum Speichern von weiteren Informationen. Über einen [Einrichterparameter](../../../firmenstamm/einrichterparameter/kontraktartikel_epa_ktrartin.md) kann das Feld weggeschaltet werden. |
| Nachhaltig | Hier kann dem Kontraktartikel ein individueller Nachhaltigkeitsstatus gegeben werden.  
Dieser Wert muss gepflegt werden, wenn im Handel zwischen zertifizierten Handelspartnern nachhaltige und nicht nachhaltige Kontrakte bei nachhaltigen Artikeln gehandelt werden. |
| [Anbauland](../../../vorgangsabwicklung/nachhaltigkeit/stammdaten/faktor_thg_wert_anbauland.md#stamm_anbauland) | Individuelles Anbauland für diesen Kontraktartikel. |
| THG-Wert Anbau | Individueller Anbau THG-Wert für diesen Kontraktartikel. |
| THG-Wert Verarbeitung | Individueller Verarbeitung THG-Wert für diesen Kontraktartikel. |
| THG-Wert Lieferung | Individueller Lieferung THG-Wert für diesen Kontraktartikel. |
| Nicht Druckrelevant | Gibt an oder legt fest, ob die dieser Artikel nicht gedruckt werden soll. |

Bei der Prüfung des Artikels auf [Nachhaltigkeit](../../../vorgangsabwicklung/nachhaltigkeit/index.md), kann per [Einrichterparameter](../../../firmenstamm/einrichterparameter/kontraktartikel_epa_ktrartin.md) (Nachhaltigkeitsüberprüfung) eingestellt werden, ob die Meldung erscheinen oder ob sie ins Fehlerprotokoll geschrieben soll.
