# Archiv-Ansichten: Technische Unterstützung

<!-- source: https://amic.de/hilfe/archivansichtentechnischeunter.htm -->

| | **Unterstützende Datenbank-Objekte** | |
| --- | --- | --- |
| View | amic_v_fa_view_profil_deliver | Liefert die von AMIC ausgelieferten Archiv-Ansichten für die Bedienerklasse -1.<br>Schlüssel favp_id, favp_besitzer aus fa_view_profil und den Kernbegriff den Profilnamen favp_name |
| View | amic_v_fa_view_profil_deliver_privat | Liefert die Privatierungen von AMIC ausgelieferten Archiv-Ansichten<br>Schlüssel favp_id, favp_besitzer aus fa_view_profil und den Kernbegriff den Profilnamen favp_name sowie den Schlüssel der dazugehörigen AMIC-Auslieferung der Ansicht |
| View | amic_v_fa_view_profil_privat | Liefert diejenigen privaten Archiv-Ansichten die weder gemäß amic_v_fa_view_profil_deliver eine von AMIC ausgelieferte Archiv-Ansicht sind noch eine daraus privatisierte Archiv-Ansicht sind (siehe auch amic_v_fa_view_profil_deliver_privat)<br>Geliefert werden Schlüssel favp_id, favp_besitzer aus fa_view_profil und der Kernbegriff Profilname favp_name |
