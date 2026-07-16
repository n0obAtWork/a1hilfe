# Datenherkunft SQL oder Relation

<!-- source: https://amic.de/hilfe/datenherkunftsqloderrelation.htm -->

Wenn man in AIS diverse Felder informatorisch anzeigt, kann man sich überlegen, ob man diese per Datenherkunft SQL ausliest oder per Datenherkunft Relation. Wird die Datenherkunft SQL gewählt, so wird für jedes Feld dieses Typs ein SQL ausgeführt. Wählt man als Datenherkunft Relation aus, wird nur einmal pro Relation das SQL ausgeführt. Man kann sich also vorstellen, dass dann bei vielen Feldern die Datenherkunft Relation besser und schneller ist.

Selbst wenn ein SQL die Daten erst zusammen baut „select trim(a.AdressVorname + ' ' a.AdressName) as InfoKundName from“, so kann man dies auch geschickt lösen, indem man sich vorher ein View aufbaut, das alle Informationsfelder enthält. Als Relation trägt man dann den Namen des Views ein.
