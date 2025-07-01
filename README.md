# Raptor CyberHealth - Chiro-Reporter
# Chiropractic Report Assistant

AI-powered medical report generation system for chiropractic practices.

## Features
- Denial Appeal Reports
- Patient Status Reports
- Permanency Reports  
- Referral Summary Reports
- OCR for scanned documents
- DICOM image analysis
- Multiple export formats (TXT, DOCX, PDF)

## Setup
1. Clone the repository
2. Create a project `.env` file and include your OpenAI API key if using OpenAI (vs other LLM)
3. Install dependencies: `pip install -r requirements.txt`
4. Run: `streamlit run streamlit_gpt_report_app.py`

## Requirements
- Rust: Logic, Microservices, endpoint for Let's Encrypt.
- OpenAI API key
- NGINX - GUI
- Optional: Tesseract OCR, Poppler, and many others needed for advanced features
  
## Security & Compliance
- Never commit patient data
- Keep API keys secure
- Follow HIPAA guidelines for patient information

## Install dependencies
pip install -r requirements.txt

## Validate environment
bash validate_chiro_report_env.sh

## Run application
whatever the command is, put it here.

