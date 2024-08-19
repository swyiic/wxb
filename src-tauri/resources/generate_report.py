import sys
import json
from docx import Document
from docx.shared import Inches
import base64
import io
import re
import os

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

def add_padding_if_needed(base64_string):
    """确保 base64 字符串长度是 4 的倍数"""
    return base64_string + "=" * ((4 - len(base64_string) % 4) % 4)

def extract_and_decode_base64_images(text):
    base64_pattern = re.compile(r'<img\s+[^>]*src=\\"data:image/[^;]+;base64,([^\\"]+)\\"[^>]*>')
    matches = base64_pattern.findall(text)
    images = []
    
    for match in matches:
        # 移除反斜杠
        match = match.replace('\\', '')
        match = add_padding_if_needed(match)
        try:
            img_data = base64.b64decode(match)
            images.append(io.BytesIO(img_data))
        except base64.binascii.Error as e:
            print(f"解码 base64 数据时出错: {e}")
    
    return images


def replace_text_in_paragraph(paragraph, replacements):
    for run in paragraph.runs:
        for placeholder, value in replacements.items():
            if placeholder in run.text:
                value = value.replace('\\"', '"')

                # 分割文本和图片部分
                parts = re.split(r'(<img\s+[^>]*src="data:image/[^;]+;base64,[^"]+"[^>]*>)', value)

                run.text = ""  # 清空现有文本内容

                for part in parts:
                    if part.startswith('<img'):
                        # 处理图片
                        img_data = re.search(r'base64,([^"]+)', part).group(1)
                        img_data = add_padding_if_needed(img_data)
                        img = base64.b64decode(img_data)
                        image_stream = io.BytesIO(img)
                        run.add_picture(image_stream, width=Inches(4))
                        run.add_break()
                    else:
                        # 处理文本部分
                        text_lines = part.replace('<div><br></div>', '\n').replace('<div>', '').replace('</div>', '').split('\n')
                        for i, line in enumerate(text_lines):
                            if i > 0:
                                run.add_break()  # 添加换行
                            run.add_text(line)


def replace_text_and_images(doc, replacements):
    # 替换段落中的文本和图片
    for paragraph in doc.paragraphs:
        replace_text_in_paragraph(paragraph, replacements)

    # 替换表格中的文本和图片
    for table in doc.tables:
        for row in table.rows:
            for cell in row.cells:
                for paragraph in cell.paragraphs:
                    print(f"正在处理表格中的段落: {paragraph.text}")  # 打印每个段落的内容
                    replace_text_in_paragraph(paragraph, replacements)
                    if 'AD' in paragraph.text:
                        print(f"AD 字段在表格中找到: {paragraph.text}")

    # 确保处理所有表格中的所有段落
    for table in doc.tables:
        for row in table.rows:
            for cell in row.cells:
                for paragraph in cell.paragraphs:
                    replace_text_in_paragraph(paragraph, replacements)


def generate_report(json_path):
    with open(json_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    print("接收到 JSON 数据，进行解析...")
    replacements = {
        "CN": data.get("company_name", ""),
        "CU": data.get("web_url", ""),
        "WT": data.get("system_name",""),
        "VN": data.get("vuln_name", ""),
        "CD": data.get("find_date", ""),
        "VU": data.get("vuln_url", ""),
        "VD": data.get("vuln_description", ""),
        "FS": data.get("fix_suggestion", ""),
        "AP": data.get("assist_image", ""),
        "TR": data.get("test_process", ""),
        "AD": data.get("appendix", "")
    }

    doc = Document("resources/demo.docx")
    replace_text_and_images(doc, replacements)

    # 获取已经存在的报告文件数
    report_files = [f for f in os.listdir() if f.endswith("--网络安全检测报告.docx")]
    next_number = len(report_files) + 1

    # 生成文件名
    output_filename = f"{next_number}-{replacements['CN']}--网络安全检测报告.docx"
    doc.save(output_filename)
    print("生成成功 ：", output_filename)

if __name__ == "__main__":
    json_path = sys.argv[1]
    generate_report(json_path)
