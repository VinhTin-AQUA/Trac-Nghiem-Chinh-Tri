import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MainRoutes } from '../../core/constants/routes-consts';
import { RouterLink } from '@angular/router';

// question.model.ts
export interface Answer {
    id: number;
    text: string;
    isCorrect: boolean;
}

export interface Question {
    id: number;
    text: string;
    answers: Answer[];
}

@Component({
    selector: 'app-question-bank',
    imports: [CommonModule, FormsModule, RouterLink],
    templateUrl: './question-bank.html',
    styleUrl: './question-bank.css',
})
export class QuestionBank {
    mainRoutes = MainRoutes;
    questions: Question[] = [
        {
            id: 1,
            text: 'Thủ đô của Việt Nam là?',
            answers: [
                { id: 1, text: 'Hà Nội', isCorrect: true },
                { id: 2, text: 'Hải Phòng', isCorrect: false },
                { id: 3, text: 'Đà Nẵng', isCorrect: false },
            ],
        },
        {
            id: 2,
            text: '2 + 5 = ?',
            answers: [
                { id: 1, text: '5', isCorrect: false },
                { id: 2, text: '7', isCorrect: true },
            ],
        },
    ];

    // SEARCH + PAGINATION
    searchTerm = '';
    page = 1;
    pageSize = 5;

    // FORM
    formMode: 'add' | 'edit' = 'add';
    showForm = false;
    showDetail = false;

    selectedQuestion: Question | null = null;

    formData: Question = {
        id: 0,
        text: '',
        answers: [],
    };

    // Filter danh sách
    get filteredQuestions() {
        return this.questions.filter((q) =>
            q.text.toLowerCase().includes(this.searchTerm.toLowerCase())
        );
    }

    // Paging
    get paginatedQuestions() {
        const start = (this.page - 1) * this.pageSize;
        return this.filteredQuestions.slice(start, start + this.pageSize);
    }

    // Mở form thêm câu hỏi
    // openAddForm() {
    //     this.formMode = 'add';
    //     this.formData = { id: 0, text: '', answers: [] };
    //     this.showForm = true;
    // }

    // Mở form edit
    openEditForm(q: Question) {
        this.formMode = 'edit';
        this.formData = JSON.parse(JSON.stringify(q)); // clone object
        this.showForm = true;
    }

    // Xem chi tiết
    openDetail(q: Question) {
        this.selectedQuestion = q;
        this.showDetail = true;
    }

    // // Thêm đáp án vào form
    // addAnswer() {
    //     this.formData.answers.push({
    //         id: Date.now(),
    //         text: '',
    //         isCorrect: false,
    //     });
    // }

    // Xóa đáp án
    removeAnswer(i: number) {
        this.formData.answers.splice(i, 1);
    }

    // Lưu form
    save() {
        if (this.formMode === 'add') {
            this.formData.id = Date.now();
            this.questions.push(JSON.parse(JSON.stringify(this.formData)));
        } else {
            const idx = this.questions.findIndex((q) => q.id === this.formData.id);
            this.questions[idx] = JSON.parse(JSON.stringify(this.formData));
        }

        this.showForm = false;
    }

    // Delete
    delete(q: Question) {
        if (confirm('Bạn có chắc muốn xóa câu hỏi này?')) {
            this.questions = this.questions.filter((x) => x.id !== q.id);
        }
    }
}
