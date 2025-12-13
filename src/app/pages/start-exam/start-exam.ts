import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { TranslatePipe } from '@ngx-translate/core';
import { Question } from '../../core/models/question';

@Component({
    selector: 'app-start-exam',
    imports: [CommonModule, FormsModule, TranslatePipe],
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
            id: 1,
            content: 'Thủ đô của Việt Nam là?',
            answers: [
                { id: 1, content: 'Hà Nội', is_correct: true, question_id: 1 },
                { id: 2, content: 'TP. HCM', is_correct: false, question_id: 1 },
                { id: 3, content: 'Đà Nẵng', is_correct: false, question_id: 1 },
            ],
        },
        {
            id: 2,
            content: '2 + 2 = ?',
            answers: [
                { id: 3, content: '3', is_correct: false, question_id: 2 },
                { id: 4, content: '4', is_correct: true, question_id: 2 },
                { id: 5, content: '5', is_correct: false, question_id: 2 },
            ],
        },
    ];

    submit() {
        this.submitted = true;

        let correct = 0;

        // this.questions.forEach((q) => {
        //     const userAns = q.answers.find((a) => a.id === q.userAnswerId);
        //     if (userAns?.is_correct) {
        //         correct++;
        //     }
        // });

        this.correctCount = correct;
        this.score = Math.round((correct / this.questions.length) * 10); // điểm thang 10
    }
}
