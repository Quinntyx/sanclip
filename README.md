## sanclip

Blazingly fast clipboard sanitizer daemon. 

Cross-platform compatible, is event-based when possible (MacOS is unfortunately implemented using polling). 

When clipboard changes, performs simple link sanitization if the copied contents are a url. 

If there are false positives or missed tracker elements, please open an issue! :)
