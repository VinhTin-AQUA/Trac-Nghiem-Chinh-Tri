import { CommonModule } from '@angular/common';
import { Component, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MainRoutes } from '../../core/constants/routes-consts';
import { RouterLink } from '@angular/router';
import { TauriCommandService } from '../../core/services/tauri-command-service';
import { TranslatePipe } from '@ngx-translate/core';
import { Question } from '../../core/models/question';
import { Answer } from '../../core/models/answer';

@Component({
    selector: 'app-question-bank',
    imports: [CommonModule, FormsModule, RouterLink, TranslatePipe],
    templateUrl: './question-bank.html',
    styleUrl: './question-bank.css',
})
export class QuestionBank {
    mainRoutes = MainRoutes;
    questions = signal<Question[]>([]);
    searchTerm = '';
    showDetail = false;
    selectedQuestion = signal<Question | null>(null);

    constructor(private tauriCommandService: TauriCommandService) {}

    async ngOnInit() {
        let questions = await this.tauriCommandService.invokeCommand<Question[]>(
            TauriCommandService.GET_ALL_QUESTIONS_COMMAND,
            {}
        );

        if (!questions) {
            return;
        }
        this.questions.set(questions);

        for (let q of this.questions()) {
            const answers = await this.tauriCommandService.invokeCommand<Answer[]>(
                TauriCommandService.GET_ANSWERS_BY_QUESTION_ID_COMMAND,
                { questionId: q.id }
            );

            if (!answers) {
                continue;
            }
            q.answers = answers;
        }
    }

    openDetail(q: Question) {
        this.selectedQuestion.set(q);
        this.showDetail = true;
    }

    delete(q: Question) {
        if (confirm('Bạn có chắc muốn xóa câu hỏi này?')) {
            this.questions.set(this.questions().filter((x) => x.id !== q.id));
        }
    }
}
