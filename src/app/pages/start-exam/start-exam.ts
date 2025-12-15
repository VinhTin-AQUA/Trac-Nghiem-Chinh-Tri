import { CommonModule } from '@angular/common';
import { Component, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { TranslatePipe } from '@ngx-translate/core';
import { Question } from '../../core/models/question';
import { TauriCommandService } from '../../core/services/tauri-command-service';
import { ToastStore } from '../../shared/stores/toast.store';
import { Answer } from '../../core/models/answer';

@Component({
    selector: 'app-start-exam',
    imports: [CommonModule, FormsModule, TranslatePipe],
    templateUrl: './start-exam.html',
    styleUrl: './start-exam.css',
})
export class StartExam {
    totalQuestionPerExam = signal<number>(20);
    toastStore = ToastStore;
    questions = signal<Question[]>([]);

    selectedAnswers: { [questionId: number]: number } = {};
    submitted = false;
    score = 0;
    correctCount = 0;

    constructor(private tauriCommandService: TauriCommandService) {}

    async ngOnInit() {
        await this.RandomQuestions();
    }

    private async RandomQuestions() {
        const questions = await this.tauriCommandService.invokeCommand<Question[]>(
            TauriCommandService.GET_RANDOM_QUESTIONS_COMMAND,
            { rand: this.totalQuestionPerExam() }
        );

        if (!questions) {
            this.toastStore.showToastMessage(true, false, 'Error', 'Cannot get quesitons');
            return;
        }
        this.questions.set(questions);

        for (let q of this.questions()) {
            const answers = await this.tauriCommandService.invokeCommand<Answer[]>(
                TauriCommandService.GET_ANSWERS_BY_QUESTION_ID_COMMAND,
                { questionId: q.id }
            );

            if (!answers) {
                q.answers = [];
            } else {
                q.answers = answers;
            }
        }

        this.selectedAnswers = {};
        this.submitted = false;
        this.score = 0;
        this.correctCount = 0;
    }

    //=========================================

    onInputTotalQuestionPerExam(num: number) {
        this.totalQuestionPerExam.set(num);
    }

    async onRandomQuestions() {
        console.log(this.totalQuestionPerExam());
        await this.RandomQuestions();
    }

    submit() {
        this.submitted = true;
        this.correctCount = 0;

        for (const q of this.questions()) {
            const selectedId = this.selectedAnswers[q.id];
            const selectedAnswer = q.answers.find((a) => a.id === selectedId);
            if (selectedAnswer?.is_correct) {
                this.correctCount++;
            }
        }

        const totalQuestions = this.questions().length;

        // Bảo vệ chia cho 0
        if (totalQuestions > 0) {
            this.score = Math.round((this.correctCount / totalQuestions) * 10 * 100) / 100;
        } else {
            this.score = 0;
        }
    }
}
