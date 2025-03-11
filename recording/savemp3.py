#!/home/botlarry/phonehome/bin/python3
import sounddevice as sd
import soundfile as sf
 
def record_microphone(duration: int = 5, filename: str = "recording.mp3"):
    """
    Function to record audio from the default microphone for a specified duration and save it as an mp3 file.
 
    Parameters:
    - duration: int (optional)
        The duration of the recording in seconds. Default is 5 seconds.
    - filename: str (optional)
        The name of the output mp3 file. Default is "recording.mp3".
 
    Raises:
    - ValueError:
        Raises an error if the duration is less than or equal to 0.
 
    Returns:
    - None
        The function does not return anything, it saves the recording as an mp3 file.
 
    """
    # Checking if the duration is valid
    if duration <= 0:
        raise ValueError("Duration should be greater than 0.")
 
    # Recording audio from the default microphone
    recording = sd.rec(int(duration sd.default.samplerate), channels=1)
 
    # Waiting for the recording to complete
    sd.wait()
 
    # Saving the recording as an mp3 file
    sf.write(filename, recording, samplerate=sd.default.samplerate, subtype="PCM_24")
 
# Example usage of the record_microphone function
record_microphone(duration=5, filename="recording.mp3")
print("Recording saved as 'recording.mp3'.")
