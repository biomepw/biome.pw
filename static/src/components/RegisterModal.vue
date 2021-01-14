<template>
  <div>
    <b-modal
      id="register-modal"
      ref="modal"
      title="Apply to the whitelist"
      @show="resetModal"
      @hidden="resetModal"
      @ok="handleOk"
    >
      <form ref="form" @submit.stop.prevent="handleSubmit">
        <b-form-group
          v-for="question in questions"
          :key="question.question"
          :state="inputState"
          :label="question.question"
          :label-for="getLabelName(question)"
        >
          <b-form-input
            :id="getIdName(question)"
            v-model="question.data"
            :state="question.state"
            required
          ></b-form-input>
          <b-form-invalid-feedback :state="validateQuestion(question)">
            {{ question.invalid }}
          </b-form-invalid-feedback>
          <b-form-valid-feedback :state="validateQuestion(question)">
            {{ question.valid }}
          </b-form-valid-feedback>
        </b-form-group>
      </form>
    </b-modal>
  </div>
</template>

<script>
export default {
  name: "RegisterModal",
  components: {},
  data: () => {
    return {
      questions: [
        {
          id: 0,
          question: "What is your Minecraft username?",
          state: null,
          data: "",
          label: "username",
          valid: "Looks good!",
          invalid: "Input must not be empty",
        },
        {
          id: 1,
          question: "How old are you?",
          state: null,
          data: "",
          label: "age",
          valid: "Looks good!",
          invalid: "Input must not be empty",
        },
        {
          id: 2,
          question:
            "What is your linking ID? (In Discord type !id in #general and BiomeBot will send you this!)",
          state: null,
          data: "",
          label: "linking-id",
          valid: "Looks good!",
          invalid: "Input must not be empty",
        },
        {
          id: 3,
          question:
            "If you could add one thing to Minecraft, what would it be?",
          state: null,
          data: "",
          label: "add-one-thing",
          valid: "Looks good!",
          invalid: "Input must not be empty",
        },
        {
          id: 4,
          question: "What are some projects you want to complete on Biome?",
          state: null,
          data: "",
          label: "projects",
          valid: "Looks good!",
          invalid: "Input must not be empty",
        },
        {
          id: 5,
          question:
            "What is your biggest Minecraft project? Or how do you spend your time playing Minecraft?",
          state: null,
          data: "",
          label: "biggest-project",
          valid: "Looks good!",
          invalid: "Input must not be empty",
        },
        {
          id: 6,
          question:
            "Can you showcase some of your previous builds? Upload here https://imgur.com/, then share the link below!",
          state: null,
          data: "",
          label: "showcase",
          valid: "Looks good!",
          invalid: "Input must not be empty",
        },
      ],
      inputState: null,
    };
  },
  methods: {
    validateUsername() {
      let question = this.questions[0];
      let url = "/validate/" + question.data;

      return this.axios.get(url).then((response) => {
        if (response.data === "") {
          question.invalid = "Please enter a valid Minecraft username!";
          question.data = "";
          return false;
        } else {
          return true;
        }
      });
    },

    validateQuestion(question) {
      let id = question.id;

      if (id === 0 || id === 3 || id === 4 || id === 5 || id === 6) {
        return question.data !== "";
      } else if (id === 1 || id === 2) {
        if (question.data.length === "") return false;
        let regex = new RegExp("^[0-9]*$");
        let numbers = regex.test(question.data);
        if (!numbers) {
          if (id === 1) {
            question.invalid = "Your age must only be numbers!";
            return false;
          } else if (id === 2) {
            question.invalid = "Discord ID must only contain numbers!";
            return false;
          }
        } else {
          if (id === 1) {
            if (question.data.length < 2) {
              question.invalid = "Age must be at minimum double digits!";
              return false;
            } else if (question.data.length > 2) {
              question.invalid = "There's no way you're that old, gramps.";
              return false;
            }
          }
          if (id === 2) {
            if (question.data.length !== 18) {
              question.invalid =
                "Discord ID must only contain numbers and must be 18 characters long!";
              return false;
            }
          }
        }

        return question.data !== "";
      }

      return false;
    },
    isValidForm() {
      let success = true;
      for (let i = 0; i < this.questions.length; i++) {
        let question = this.questions[i];

        if (!this.validateQuestion(question)) {
          success = false;
        }
      }

      return success;
    },
    handleOk(bvModalEvt) {
      bvModalEvt.preventDefault();
      if (this.isValidForm()) {
        this.handleSubmit();
      }
    },
    handleSubmit() {
      this.validateUsername().then((result) => {
        // If it failed, bail!
        if (!result) return;

        // Post application submission
        this.axios
          .post("/application/submit", {
            minecraftUsername: this.questions[0].data,
            age: parseInt(this.questions[1].data),
            linkingId: parseInt(this.questions[2].data),
            addOneThing: this.questions[3].data,
            projectsOnBiome: this.questions[4].data,
            biggestProject: this.questions[5].data,
            showcase: this.questions[6].data,
          })
          .then((response) => {
            if (response.data !== "Application inserted successfully.") {
              this.$emit("application-fail", response.data);
            } else {
              this.$emit("successful-registration");
            }
            this.$bvModal.hide("register-modal");
          })
          .catch((error) => {
            this.$emit("application-fail", "Caught an error! - " + error);
            this.$bvModal.hide("register-modal");
          });
      });
    },
    resetModal() {
      for (let i = 0; i < this.questions.length; i++) {
        this.questions[i].data = "";
      }
    },
    getLabelName(question) {
      return question.label + "-label";
    },
    getIdName(question) {
      return question.label + "-input";
    },
  },
};
</script>

<style scoped>
</style>
