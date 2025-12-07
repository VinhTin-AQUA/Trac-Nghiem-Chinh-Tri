import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
    selector: 'app-add-question',
    imports: [CommonModule, FormsModule],
    templateUrl: './add-question.html',
    styleUrl: './add-question.css',
})
export class AddQuestion {
    questionText = '';

    answers = [{ id: Date.now(), text: '', isCorrect: false }];

    // thêm đáp án
    addAnswer() {
        this.answers.push({
            id: Date.now() + Math.random(),
            text: '',
            isCorrect: false,
        });
    }

    // chọn đáp án đúng (đảm bảo chỉ một đáp án)
    markCorrect(answerId: number) {
        this.answers.forEach((a) => (a.isCorrect = false));
        const found = this.answers.find((a) => a.id === answerId);
        if (found) found.isCorrect = true;
    }

    // xóa đáp án
    removeAnswer(index: number) {
        this.answers.splice(index, 1);
    }

    // lưu câu hỏi
    save() {
        if (!this.questionText.trim()) {
            alert('Vui lòng nhập nội dung câu hỏi.');
            return;
        }
        if (this.answers.length < 2) {
            alert('Cần ít nhất 2 đáp án.');
            return;
        }
        if (!this.answers.some((a) => a.isCorrect)) {
            alert('Bạn phải chọn 1 đáp án đúng.');
            return;
        }

        const newQuestion = {
            id: Date.now(),
            text: this.questionText,
            answers: this.answers,
        };

        console.log('Câu hỏi đã thêm:', newQuestion);

        alert('Thêm câu hỏi thành công!');

        // Reset form
        this.questionText = '';
        this.answers = [{ id: Date.now(), text: '', isCorrect: false }];
    }
}
