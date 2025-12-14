import { Answer } from '../../core/models/answer';
import { Question } from '../../core/models/question';

export class EditQuestionStore {
    static question: Question | null = null;
    static answers: Answer[] = [];
}
