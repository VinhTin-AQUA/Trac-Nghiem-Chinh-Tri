import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';

// question.model.ts
export interface Answer {
    id: string;
    text: string;
    isCorrect: boolean;
}

export interface Question {
    id: string;
    text: string;
    answers: Answer[];
    userAnswerId?: string; // đáp án người dùng chọn
}

@Component({
    selector: 'app-start-exam',
    imports: [CommonModule, FormsModule],
    templateUrl: './start-exam.html',
    styleUrl: './start-exam.css',
})
export class StartExam {
    submitted = false;
    score = 0;
    correctCount = 0;

    questions2: Question[] = [];
    questions: Question[] = [
        {
            id: crypto.randomUUID(),
            text: 'Thủ đô của Việt Nam là?',
            answers: [
                { id: crypto.randomUUID(), text: 'Hà Nội', isCorrect: true },
                { id: crypto.randomUUID(), text: 'TP. HCM', isCorrect: false },
                { id: crypto.randomUUID(), text: 'Đà Nẵng', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: '2 + 2 = ?',
            answers: [
                { id: crypto.randomUUID(), text: '3', isCorrect: false },
                { id: crypto.randomUUID(), text: '4', isCorrect: true },
                { id: crypto.randomUUID(), text: '5', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: 'Thủ đô của Việt Nam là?',
            answers: [
                { id: crypto.randomUUID(), text: 'Hà Nội', isCorrect: true },
                { id: crypto.randomUUID(), text: 'TP. HCM', isCorrect: false },
                { id: crypto.randomUUID(), text: 'Đà Nẵng', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: '2 + 2 = ?',
            answers: [
                { id: crypto.randomUUID(), text: '3', isCorrect: false },
                { id: crypto.randomUUID(), text: '4', isCorrect: true },
                { id: crypto.randomUUID(), text: '5', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: 'Thủ đô của Việt Nam là?',
            answers: [
                { id: crypto.randomUUID(), text: 'Hà Nội', isCorrect: true },
                { id: crypto.randomUUID(), text: 'TP. HCM', isCorrect: false },
                { id: crypto.randomUUID(), text: 'Đà Nẵng', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: '2 + 2 = ?',
            answers: [
                { id: crypto.randomUUID(), text: '3', isCorrect: false },
                { id: crypto.randomUUID(), text: '4', isCorrect: true },
                { id: crypto.randomUUID(), text: '5', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: 'Thủ đô của Việt Nam là?',
            answers: [
                { id: crypto.randomUUID(), text: 'Hà Nội', isCorrect: true },
                { id: crypto.randomUUID(), text: 'TP. HCM', isCorrect: false },
                { id: crypto.randomUUID(), text: 'Đà Nẵng', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: '2 + 2 = ?',
            answers: [
                { id: crypto.randomUUID(), text: '3', isCorrect: false },
                { id: crypto.randomUUID(), text: '4', isCorrect: true },
                { id: crypto.randomUUID(), text: '5', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: 'Thủ đô của Việt Nam là?',
            answers: [
                { id: crypto.randomUUID(), text: 'Hà Nội', isCorrect: true },
                { id: crypto.randomUUID(), text: 'TP. HCM', isCorrect: false },
                { id: crypto.randomUUID(), text: 'Đà Nẵng', isCorrect: false },
            ],
        },
        {
            id: crypto.randomUUID(),
            text: '2 + 2 = ?',
            answers: [
                { id: crypto.randomUUID(), text: '3', isCorrect: false },
                { id: crypto.randomUUID(), text: '4', isCorrect: true },
                { id: crypto.randomUUID(), text: '5', isCorrect: false },
            ],
        },
    ];

    submit() {
        this.submitted = true;

        let correct = 0;

        this.questions.forEach((q) => {
            const userAns = q.answers.find((a) => a.id === q.userAnswerId);
            if (userAns?.isCorrect) {
                correct++;
            }
        });

        this.correctCount = correct;
        this.score = Math.round((correct / this.questions.length) * 10); // điểm thang 10
    }
}
