# Archiv-Manager

<!-- source: https://amic.de/hilfe/_archivmanager.htm -->

Hauptmenü > Administration > Archiv > Verwaltung

oder Direktsprung **[FAM]**

Im Archiv-Manager werden alle Einstellungen, Optionen, Parameter bezüglich des Archivs eingerichtet.

| Felder | | |
| --- | --- | --- |
| Lizenz | Information | Gibt an, ob eine Lizenz für das Archiv installiert ist. |
| Archivieren | Pflichtfeld | Schalter, ob eine Archivierung stattfinden soll.<br>Diese Möglichkeit ist sollte nur in Ausnahmefällen auf NEIN gestellt werden. |
| Ziel | Pflichtfeld | A.eins unterstützt ausschließlich die Archivierung in die Datenbank. |
| EXTERN | Information | Ist die Relation ARCHIV extern angeschlossen, wird hier die Information gegeben, welche System-Bindungen vorliegen.<br>**Zusätzlich wird noch der aktuelle Status, ob die Relation ARCHIV zugreifbar ist, bekanntgegeben.**<br>Bedeutung der technischen Information:<br>remote-server-name;db-name;owner;object-name<br>#<br>srvname/srvclass/srvinfo<br> <br>Es sind zur administrativen Unterstützung folgende Funktionen in der Optionbox verfügbar gemacht worden:<br><ul><li>&nbsp;&nbsp;&nbsp; Sybase Central</li><li>&nbsp;&nbsp;&nbsp; ODBC Administrator<br>&nbsp;<br>Erinnerung/Hinweis: Windows unterscheidet beim Datenquellennamen (DSN) Groß-/Kleinschreibung. Außerdem sind externe Anbindungen in aller Regel Systemdatenquellen, also solche die via „System-DSN“ eingerichtet sind.</li></ul> |

| Funktionen | | |
| --- | --- | --- |
| Archiv-Fakte | SF9 | Siehe [Archiv2.docx#ueb_Archivfakte](../archiv_fakte.md) |
| Ansichten | F6 | Siehe [Archiv Anichten](../archiv_ansehen/archiv_ansichten_variante_ansichten/index.md) |
| Recherche-Funktionen … | F5 | Erlaubt das direkte Pflegen der Datenbank-Recherche-Funktion (Datenbank-Recherche) |
| ODBC-Administrator … | SF10 | Ruft den [ODBC-Datenquellen-Administrator](http://technet.microsoft.com/de-de/library/ms188691.aspx) Ihres Systems auf. |
| Externes Archiv abkoppeln | | Diese Funktion ist nur dann erreichbar, wenn das Archiv extern angebunden wurde.<br>Sie können mit dieser Funktion die Relation „Archiv“ wieder intern herstellen. **Die Relation Archiv ist dann leer**, so dass ein bestehendes Formulararchiv keine Dokumente auf diesem Wege erreichen kann. |

| Registerkarten | |
| --- | --- |
| Container | [Einstellungen zum Container-Modell](./container.md) |
| Archivierung | [Archivierungsmerkmale der Dokumente](./archivierungsmerkmale_der_dokumente.md) |
| Ansehen | [Einstellungen zum „Archiv ansehen“](./ansehen/index.md) |
| XML Ex- und Import | |
| Auslagerung | |
| AMICAR | |
| Sonstiges | [Einstellungen zum „Archiv ansehen“](./ansehen/index.md) |
| Volltext-Recherche | [Archiv Volltext-Recherche](./referenz/archiv_volltext_recherche.md) |

<p class="siehe-auch">Siehe auch:</p>

- [Container](./container.md)
- [Einrichten eines Containers](./einrichten_eines_containers.md)
- [Archivierungsmerkmale der Dokumente](./archivierungsmerkmale_der_dokumente.md)
- [Archiv-Manager Sonstiges](./archiv_manager_sonstiges/index.md)
- [Ansehen](./ansehen/index.md)
- [Referenz](./referenz/index.md)
- [Archiv Mail Versand](./archiv_mail_versand/index.md)
- [Signature Pad einrichten](./signature_pad_einrichten/index.md)
- [Signature Pad benutzen](./signature_pad_benutzen/index.md)
