# ZMDO mehrere Kunden mit gleicher USTID akzeptieren (SPA 934)

<!-- source: https://amic.de/hilfe/_SPA_934.htm -->

Für die Zusammenfassende Meldung (Direktsprung [UVZM]) werden die Daten zuerst in einer Auswahlliste –Variante „Zusammenfassende Meldung nach AWPosition“ - zusammengestellt und diese Informationen von dort an die entsprechenden Übertragungs- bzw. Informationsbereiche weitergeleitet. Die Auswahlliste generiert pro Konto die Daten. Dies ist die Standardeinstellung dieses Steuerungsparameters: **Nein**. Wenn zu unterschiedlichen Konten dieselbe UDTID hinterlegt ist – weil es sich z.B. um verschiedene Filialen handelt – kommt es bei der Übertragung zu Problemen, da dieselbe USTID nicht mehrfach ( es sei denn mit anderen Kennzeichen für Dreiecksgeschäft / Sonstige Leistung) vorkommen darf. Stellt man den Steuerungsparameter auf **Ja**, so werden die Daten nach der USTID gruppiert. Die Konten werden dann nur noch Informatorisch angezeigt.
